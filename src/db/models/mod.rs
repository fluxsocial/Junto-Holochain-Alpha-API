use actix_web::{error::Error as AWError};
use validator::Validate;
use uuid::Uuid;
use diesel::prelude::*;
use bcrypt::verify;

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
    pub last_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HolochainUserRequst{
    pub params: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HolochainRequest{
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HolochainResponse{
    pub id: String,
    pub jsonrpc: String,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name= "users" ]
pub struct Users {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub pub_address: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

impl From<Users> for SlimUser {
    fn from(user: Users) -> Self {
        SlimUser { id: user.id.to_string() }
    }
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

    pub fn get_pub_key<'a>(user_id: &'a str, pool: &Pool) -> Result<String, errors::JuntoApiError> {
        use crate::schema::users::dsl::*;
        let user_id = Uuid::parse_str(user_id).unwrap();
        let conn: Connection = pool.get().unwrap();
        match users.select(pub_address).filter(id.eq(user_id)).first::<String>(&conn) {
            Ok(entry) => Ok(entry),
            Err(_err) => Err(errors::JuntoApiError::InternalError)
        }
    }

    pub fn can_login(auth_data: AuthData, pool: &Pool) -> Result<SlimUser, errors::JuntoApiError> {
        use crate::schema::users::dsl::*;
        let conn: Connection = pool.get().unwrap();

        let mut items = users
            .filter(email.eq(&auth_data.email))
            .load::<Users>(&conn).map_err(|_err| errors::JuntoApiError::InternalError)?;

        if let Some(user) = items.pop() {
            if let Ok(matching) = verify(&user.password, &auth_data.password) {
                if matching {
                    return Ok(user.into()); // convert into slimUser
                }
            }
        }
        Err(errors::JuntoApiError::Unauthorized)
    }
}