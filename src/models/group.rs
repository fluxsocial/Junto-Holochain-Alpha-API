//Holochain request data types
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePackRequestData {
    pub username_address: String, 
    pub first_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddPackMemberRequestData {
    pub username_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddMemberToGroupRequestData {
    pub username_address: String,
    pub group: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveGroupMemberRequestData {
    pub username_address: String,
    pub group: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetGroupMembersRequestData {
    pub group: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IsGroupOwnerRequestData {
    pub username_address: String,
    pub group: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IsGroupMemberRequestData {
    pub username_address: String,
    pub group: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserPacksRequestData {
    pub username_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserMemberPacksRequestData {
    pub username_address: String,
}
