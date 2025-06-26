use std::sync::{Arc, Mutex};


use poem::{
    get, listener::TcpListener, post, EndpointExt, Route, Server
};

use store::store::Store;
pub mod request_input;
pub mod request_output;
pub mod routes;
use crate::{
     routes::{user::{sign_in, sign_up}, website::{create_website, get_website}},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::default().unwrap()));
    let app = Route::new()
        .at("/signup", post(sign_up))
        .at("/signin", post(sign_in))
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .data(s);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
