use crate::{ models::website, store::Store };
use diesel::{ dsl::Values, prelude::* };
use uuid::Uuid;
use chrono::{ NaiveDateTime, Utc };

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::Website)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Websites {
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
    ) -> Result<Websites, diesel::result::Error> {
        let id: Uuid = Uuid::new_v4();

        let website = Websites {
            user_id,
            url,
            id: id.to_string(),
            timeAdded: Utc::now().naive_local(),
        };

        let website_res_ = diesel
            ::insert_into(crate::schema::Website::table)
            .values(&website)
            .returning(Websites::as_returning())
            .get_result(&mut self.conn)?;

        Ok(website_res_)
    }

    pub fn get_website(&mut self, input_id: String) -> Result<Websites, diesel::result::Error> {
        use crate::schema::Website::dsl::*;

        let website_res_ = Website.filter(id.eq(input_id))
            .select(Websites::as_select())
            .first(&mut self.conn)?;

        Ok(website_res_)
    }
}
