pub mod collection;
pub mod config;
pub mod expression;
pub mod group;
pub mod persepective;
pub mod user;
pub mod db;

//General holochain request data types
#[derive(Debug, Deserialize, Serialize)]
pub enum Zomes {
    Collection,
    Config,
    Expression,
    Group,
    Perspective,
    User
}

pub enum CollectionZomeFunctions{

}

pub enum ConfigZomeFunctions{
    
}

pub enum ExpressionZomeFunctions{
    
}

pub enum GroupZomeFunctions{
    
}

pub enum PerZomeFunctions{
    
}

pub enum UserZomeFunctions{
    
}

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