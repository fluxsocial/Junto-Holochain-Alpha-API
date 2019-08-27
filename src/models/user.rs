use validator::Validate;

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

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}