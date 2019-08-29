use serde_json::Value;

pub mod user;
pub mod db;

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
    pub args: Value,//no mapping from user request data -> holochain types using serde - instead will be done manually by passing string in order; zome -> function -> args
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

impl HolochainRequest{
    pub fn from_user_req(request: HolochainUserRequest, pub_key: String) -> Result<Self, serde_json::Error>{
        let req_params = HolochainUserRequestWithInstance{args: serde_json::from_str(request.args.as_str())?, function: request.function, 
            zome: request.zome, instance_id: format!("junto-app-{}", pub_key)};

        Ok(HolochainRequest{id: pub_key, jsonrpc: "2.0".to_string(), method: "call".to_string(), params: req_params})
    }
}