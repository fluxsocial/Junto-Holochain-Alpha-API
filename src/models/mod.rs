use serde_json::Value;

pub mod user;
pub mod db;

//API -> Holochain conductor request data types
#[derive(Debug, Deserialize, Serialize)]
pub struct HolochainUserRequest{
    pub args: String,
    pub function: String,
    pub zome: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HolochainUserRequestWithInstance{
    pub args: Value,
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
            zome: request.zome, instance_id: format!("junto-{}", pub_key)};

        Ok(HolochainRequest{id: pub_key, jsonrpc: "2.0".to_string(), method: "call".to_string(), params: req_params})
    }
}