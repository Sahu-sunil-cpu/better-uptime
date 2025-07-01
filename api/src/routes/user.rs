use std::{collections::HashMap, sync::{Arc, Mutex}};

use poem::{
    Error, handler,
    http::StatusCode,
    web::{Data, Json},
};


use store::store::Store;

use crate::request_output::{CreateUserInput, CreateUserOutput};


#[handler]
pub fn sign_up(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<CreateUserOutput>, Error> {
    let mut locked_s = s.lock().unwrap();
    let user = locked_s
        .sign_up(data.username, data.password);

    match user {
        Ok(user) => {

    let res = CreateUserOutput { 
        res: user 
    };

    Ok(Json(res))
        }

        Err(e) => {
        print!("{}", e);
        return Err(Error::from_status(StatusCode::CONFLICT));
            
    }
  // propogates error if in case otherwise does remaining stuffs

}
}

#[handler]
pub fn sign_in(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<CreateUserOutput>, Error> {
    let mut locked_s = s.lock().unwrap();

    let user_id = locked_s.sign_in(data.username, data.password);

    match user_id {
        Ok(user_id) => {
            let response = CreateUserOutput { res: user_id };

            return Ok(Json(response));
        }

        Err(_) => {
            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
        }
    }
}
