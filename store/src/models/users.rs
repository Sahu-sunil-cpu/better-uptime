use crate::{ schema::user, store::Store };
use diesel::prelude::*;
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
    ) -> Result<bool, diesel::result::Error> {
        use crate::schema::user::dsl::*;

        let result = user
            .filter(username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)?;


            // password hashing checks
        if result.password != input_password {
            return Ok(false)
        }

        Ok(true)
    }
}
