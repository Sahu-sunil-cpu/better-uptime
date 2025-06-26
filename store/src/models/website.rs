use crate::{schema::website, store::Store};
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: String,
    pub url: String,
    pub timeAdded: chrono::NaiveDateTime,
    pub userId: String,
}

impl Store {
    pub fn create_website(
        &mut self,
        user_id: String,
        url: String,
    ) -> Result<Website, diesel::result::Error> {
        let id = Uuid::new_v4();

        let website = Website {
            id: id.to_string(),
            url: url,
            timeAdded: Utc::now().naive_utc(),
            userId: user_id,
        };

        let website = diesel::insert_into(website::table)
            .values(&website)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)?;

        Ok(website)
    }
    pub fn get_website(
        &mut self,
        user_id: String,
    ) -> Result<Website, diesel::result::Error> {
        use crate::schema::website::dsl::*;

        let result = website
            .filter(userId.eq(user_id))
            .select(Website::as_select())
            .first(&mut self.conn)?;

        Ok(result)
    }
}
