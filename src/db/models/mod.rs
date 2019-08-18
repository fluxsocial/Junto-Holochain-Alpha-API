use actix_web::web;
use validator::Validate;
use uuid::Uuid;
use diesel::result::Error;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::users;
use crate::db::Pool;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterData{
    #[validate(email)]
    pub email: String,
    pub password: String,
    #[validate(length(min = 1, max = 20))]
    pub first_name: String,
    #[validate(length(min = 1, max = 20))]
    pub last_name: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name= "users" ]
pub struct Users {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub pub_address: String,
    pub first_name: String,
    pub last_name: String
}

impl Users {
    pub fn is_pub_address_in_use(comp_pub_key: String, pool: web::Data<Pool>) -> Result<bool, Error> {
        use crate::schema::users::dsl::*;

        let conn: &PgConnection = &pool.get().unwrap();
        match users.filter(pub_address.eq(comp_pub_key)).first::<Users>(&*conn) {
            Ok(_entry) => Ok(true),
            Err(_err) => Ok(false)
        }
    }

    pub fn number_of_users(pool: web::Data<Pool>) -> Result<usize, Error> {
        let conn: &PgConnection = &pool.get().unwrap();
        let users_count = users::table.load::<Users>(&*conn)?;
        Ok(users_count.len())
    }
}