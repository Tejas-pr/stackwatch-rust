use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User {
    id: String,
    name: String,
    password: String,
}

impl Store {
    pub fn sign_up(&mut self, name: String, password: String) -> Result<String, diesel::result::Error> {
        use crate::schema::user;

        let id = Uuid::new_v4().to_string();

        let u = User {
            id: id.clone(),
            name,
            password,
        };
        diesel
            ::insert_into(user::table)
            .values(&u)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;

        Ok(id.to_string())
    }

    pub fn sign_in(&self, username: String, password: String) {}
}
