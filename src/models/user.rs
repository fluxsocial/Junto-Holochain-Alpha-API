use validator::Validate;

//API request data
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

//Holochain Types
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserInformation{
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: String,
    pub bio: String
}

//Holochain request data
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserRequestData {
    pub user_data: CreateUserInformation
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUsernameFromAddressRequestData {
    pub username_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserProfileFromAddressRequestData {
    pub username_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserProfileByAgentAddressRequestData {
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserUsernameByAgentAddressRequestData {
}