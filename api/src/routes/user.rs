use std::sync::{Arc, Mutex};


use poem::{
    handler,  web::{Data, Json}
};

use store::store::Store;

use crate::{
    request_output::{CreateUserInput, CreateUserOutput},
};


#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateUserOutput> {
    let mut locked_s = s.lock().unwrap();
    let user = locked_s.sign_up(data.username, data.password).unwrap();
    let res = CreateUserOutput { res: user };
    Json(res)
}

#[handler]
pub fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateUserOutput> {
   let mut locked_s = s.lock().unwrap();
   
    locked_s.sign_in(data.username, data.password).unwrap();

    //jwt and some other checks
    let res = CreateUserOutput {
        res: String::from("Successfully Logined"),
    };
    Json(res)
}
