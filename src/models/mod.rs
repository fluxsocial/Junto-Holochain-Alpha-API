pub mod collection;
pub mod config;
pub mod expression;
pub mod group;
pub mod persepective;
pub mod user;
pub mod db;

//Holochain zomes
#[derive(Debug, Deserialize, Serialize)]
pub enum Zomes {
    Collection,
    Config,
    Expression,
    Group,
    Perspective,
    User,
}

//Holochain zome functions present in each zome
#[derive(Debug, Deserialize, Serialize)]
pub enum CollectionZomeFunctions{
    CreateDen,
    GetUserDens,
    IsCollectionOwner,
    CreateCollection,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ConfigZomeFunctions{
    GetEnv,
    GetCurrentBitPrefix,
    UpdateBitPrefix,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ExpressionZomeFunctions{
    QueryExpression,
    GetExpression,
    PostExpression,
    PostCommentExpression,
    PostResonation,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GroupZomeFunctions{
    CreatePack,
    AddPackMember,
    AddMemberToGroup,
    RemoveGroupMember,
    GetGroupMembers,
    IsGroupOwner,
    IsGroupMember,
    GetUserPacks,
    GetUserMemberPacks,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PerspectiveZomeFunctions{
    CreatePerspective,
    AddUserToPerspective,
    GetPerspectiveUsers,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum UserZomeFunctions{
    CreateUser,
    GetUsernameFromAddress,
    GetUserProfileFromAddress,
    GetUserProfileByAgentAddress,
    GetUserUsernameByAgentAddress,
}

//Holochain privacy type
#[derive(Serialize, Deserialize, Debug)]
pub enum Privacy {
    Public, //Viewable by everyone
    Shared, //Viewable by selected people
    Private //Viewable by only owner
}

//API -> Holochain conductor request data types
#[derive(Debug, Deserialize, Serialize)]
pub struct HolochainUserRequest{
    pub args: String,
    pub function: String,
    pub zome: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HolochainUserRequestWithInstance{
    pub args: String,
    pub function: String,
    pub zome: String,
    pub instance_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HolochainRequest{
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: HolochainUserRequestWithInstance,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HolochainResponse{
    pub id: String,
    pub jsonrpc: String,
    pub result: String,
}

impl HolochainUserRequestWithInstance{
    pub fn from_user_req(request: HolochainUserRequest, pub_key: String) -> Self {
        HolochainUserRequestWithInstance{args: request.args, function: request.function, zome: request.zome, instance_id: format!("junto-app-{}", pub_key)}
    }
}

impl HolochainRequest{
    pub fn from_user_req(request: HolochainUserRequest, pub_key: String) -> Self{
        let req_params = HolochainUserRequestWithInstance{args: request.args, function: request.function, 
            zome: request.zome, instance_id: format!("junto-app-{}", pub_key)};

        HolochainRequest{id: pub_key, jsonrpc: "2.0".to_string(), method: "call".to_string(), params: req_params}
    }
}