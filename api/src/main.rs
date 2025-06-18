use poem::{
    Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{Json, Path},
};
use request_input::CreateWebsiteOutput;
use request_output::CreateWebsiteInput;
pub mod connection;
pub mod request_input;
pub mod request_output;

use crate::{
    connection::get_store,
    request_output::{CreateUserInput, CreateUserOutput},
};

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput> {
    let user = get_store().sign_up(data.username, data.password).unwrap();
    let res = CreateUserOutput { res: user };
    Json(res)
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput> {
    get_store().sign_in(data.username, data.password).unwrap();
    let res = CreateUserOutput {
        res: String::from("Successfully Logined"),
    };
    Json(res)
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let website = get_store()
        .create_website(
            String::from("83c524bc-0ffa-44cd-a1bc-2483246182a6"),
            data.url,
        )
        .unwrap();

    let res = CreateWebsiteOutput { id: website.id };
    Json(res)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/signup", post(sign_up))
        .at("/signin", post(sign_in))
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
