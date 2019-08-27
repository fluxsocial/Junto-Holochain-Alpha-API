#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePerspectiveRequestData {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddUserToPerspectiveRequestData {
    pub perspective: String,
    pub target_user: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetPerspectiveUsersRequestData {
    pub persepective: String,
}