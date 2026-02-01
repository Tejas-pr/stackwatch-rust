use crate::store::Store;
use diesel::{ dsl::Values, prelude::* };
use uuid::Uuid;
use chrono::{ NaiveDateTime, Utc };

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::Website)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: String,
    pub url: String,
    pub timeAdded: NaiveDateTime,
    pub user_id: String,
}

impl Store {
    pub fn create_website(
        &mut self,
        user_id: String,
        url: String
    ) -> Result<Website, diesel::result::Error> {
        let id: Uuid = Uuid::new_v4();

        let website = Website {
            user_id,
            url,
            id: id.to_string(),
            timeAdded: Utc::now().naive_local(),
        };

        let website_ = diesel
            ::insert_into(crate::schema::Website::table)
            .values(&website)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)?;

        Ok(website)
    }

    pub fn get_website(&self) {}
}
