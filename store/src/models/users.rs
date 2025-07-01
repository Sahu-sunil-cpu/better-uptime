use core::panic;
use std::collections::HashMap;

use crate::{schema::user, store::Store};
use diesel::prelude::*;
use jsonwebtoken::{EncodingKey, Header, encode, errors::Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User {
    id: String,
    username: String,
    password: String,
}

impl Store {
    pub fn sign_up(
        &mut self,
        username: String,
        password: String,
    ) -> Result<String, diesel::result::Error> {
        let id = Uuid::new_v4();

        let new_user = User {
            id: id.to_string(),
            username,
            password,
        };

        diesel::insert_into(user::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;

        Ok(id.to_string())
    }

    pub fn sign_in(
        &mut self,
        input_password: String,
        input_username: String,
    ) -> Result<String, diesel::result::Error> {
        use crate::schema::user::dsl::*;

        let result = user
            .filter(username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)?;

        // password hashing checks

        let _jw_t = match jwt(result.id) {
            Ok(s) => {
                if result.password == input_password {
                    return Ok(s);
                }
                 print!("yes");
                return Err(diesel::result::Error::NotFound);
            }

            Err(e) => {
                                 print!("yes");

                println!("{:?}", e);

                panic!() // need to have ac ustom error for this, panic is bad
            }
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn jwt(user_id: String) -> Result<String, jsonwebtoken::errors::ErrorKind> {
    let my_claims = Claims {
        sub: user_id,
        exp: 1111111111111111,
    };

    let key = b"secret";

    // let mut extras = HashMap::with_capacity(1);
    // extras.insert("custom".to_string(), "header".to_string());

    //   let header = Header {
    //     kid: Some("signing_key".to_owned()),
    //     alg: Algorithm::HS512,
    //     extras,
    //     ..Default::default()
    // };

    let _token = match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key),
    ) {
        Ok(t) => {
            println!("{:?}", t);

            return Ok(t);
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(Error::into_kind(e));
        } // in practice you would return the error
    };
}
