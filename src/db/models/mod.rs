use actix_web::{error::Error as AWError};
use validator::Validate;
use uuid::Uuid;
use diesel::prelude::*;

use crate::schema::users;
use crate::db::{Connection, Pool};
use crate::errors;

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
    pub fn is_pub_address_in_use(comp_pub_key: String, pool: &Pool) -> Result<bool, AWError> {
        use crate::schema::users::dsl::*;

        let conn: Connection = pool.get().unwrap();
        match users.filter(pub_address.eq(comp_pub_key)).first::<Users>(&conn) {
            Ok(_entry) => Ok(true),
            Err(_err) => Ok(false)
        }
    }

    pub fn number_of_users(pool: &Pool) -> Result<usize, errors::JuntoApiError> {
        let conn: Connection = pool.get().unwrap();
        let users_count = users::table.load::<Users>(&conn).map_err(|err| {
            println!("Error: {}", err); //log err
            errors::JuntoApiError::InternalError
        })?;
        Ok(users_count.len())
    }

    pub fn insert_user(user: &Users, pool: &Pool) -> Result<(), AWError>{
        let conn: Connection = pool.get().unwrap();
        let _result: Users = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)
            .unwrap();
        Ok(())
    }
}