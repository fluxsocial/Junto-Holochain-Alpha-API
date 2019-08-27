use std::fs;

use crate::models::{self, db};
use crate::errors;

static KEY_DIR: &'static str = "/home/josh/.config/holochain/keys/";
static HC_CONDUCTOR: &'static str = "http://localhost:4000";

pub fn assign_agent(pool: &db::Pool) -> Result<String, errors::JuntoApiError> {
    let possible_keys: Vec<_> = fs::read_dir(KEY_DIR).unwrap().map(|res| res.unwrap().path()).collect();
    let n_users = db::user::Users::number_of_users(pool).map_err(|_err| errors::JuntoApiError::InternalError)?;
    println!("Number of users: {}", n_users);
    let split_path = possible_keys[n_users+1].to_str().unwrap().split("/").collect::<Vec<&str>>();
    Ok(split_path[split_path.len()-1].to_string())
}

pub fn call_holochain(data: models::HolochainUserRequest, pub_address: String) -> Result<models::HolochainResponse, errors::JuntoApiError>{
    let data = models::HolochainRequest::from_user_req(data, pub_address);
    println!("Request data bef: {:?}", data);
    //let request_data = serde_json::to_string(&data).map_err(|_err| errors::JuntoApiError::InternalError)?;
    let client = reqwest::Client::new();
    let mut res = client.post(HC_CONDUCTOR)
                .header("context-type", "application/json")
                .json(&data)
                .send()
                .map_err(|err| {
                    println!("Holochain request Err {:?}", err);
                    errors::JuntoApiError::InternalError
                })?;
    println!("Response: {:?}", res);
    let response_data = res.json().map_err(|err| {
        println!("Error in response ser: {:?}", err);
        errors::JuntoApiError::InternalError})?;
    Ok(response_data)
}