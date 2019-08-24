use std::fs;

use crate::db::{models, Pool};
use crate::errors;

static KEY_DIR: &'static str = "/home/josh/.config/holochain/keys/";
static HC_CONDUCTOR: &'static str = "http://localhost:8888";

pub fn assign_agent(pool: &Pool) -> Result<String, errors::JuntoApiError> {
    let possible_keys: Vec<_> = fs::read_dir(KEY_DIR).unwrap().map(|res| res.unwrap().path()).collect();
    let n_users = models::Users::number_of_users(pool).map_err(|_err| errors::JuntoApiError::InternalError)?;
    println!("Number of users: {}", n_users);
    let split_path = possible_keys[n_users+1].to_str().unwrap().split("/").collect::<Vec<&str>>();
    Ok(split_path[split_path.len()-1].to_string())
}

pub fn call_holochain(data: models::HolochainUserRequst, pub_address: String) -> Result<models::HolochainResponse, errors::JuntoApiError>{
    let data = models::HolochainRequest::from_user_req(data, pub_address);
    let request_data = serde_json::to_string(&data).map_err(|_err| errors::JuntoApiError::InternalError)?;
    let client = reqwest::Client::new();
    let mut res = client.post(HC_CONDUCTOR)
                .header("context-type", "application/json")
                .body(request_data)
                .send()
                .map_err(|_err| errors::JuntoApiError::InternalError)?;
    let response_data: models::HolochainResponse = res.json().map_err(|_err| errors::JuntoApiError::InternalError)?;
    Ok(response_data)
}