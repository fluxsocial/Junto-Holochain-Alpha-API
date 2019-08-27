use crate::models;

//Holochain data types
#[derive(Debug, Deserialize, Serialize)]
pub struct Collection {
    pub parent: String,
    pub name: String,
    pub privacy: models::Privacy //Privacy enum 
}

//Holochain request data types
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDenRequestData {
    pub username_address: String, 
    pub first_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserDenRequestData {
    pub username_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IsCollectionOwnerRequestData {
    pub collection: String, 
    pub username_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCollectionRequestData {
    pub collection: Collection, 
    pub collection_tag: String,
}