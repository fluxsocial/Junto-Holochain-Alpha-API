use validator::Validate;
use uuid::Uuid;
use crate::schema::users;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterData{
    #[validate(email)]
    email: String,
    password: String,
    #[validate(length(min = 1, max = 20))]
    first_name: String,
    #[validate(length(min = 1, max = 20))]
    last_name: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name= "users" ]
pub struct Users {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub pub_address: String,
}
