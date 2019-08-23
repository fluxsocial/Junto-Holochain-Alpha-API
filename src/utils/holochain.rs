use std::fs;

use crate::db::{models, Pool};
use crate::errors;

static KEY_DIR: &'static str = "/Users/Josh/.config/holochain/keys";
static HC_CONDUCTOR: &'static str = "http://localhost:8888";

pub fn assign_agent(pool: &Pool) -> Result<String, errors::JuntoApiError> {
    let possible_keys: Vec<_> = fs::read_dir(KEY_DIR).unwrap().map(|res| res.unwrap().path()).collect();
    let n_users = models::Users::number_of_users(pool).map_err(|_err| errors::JuntoApiError::InternalError)?;
    println!("Number of users: {}", n_users);
    let split_path = possible_keys[n_users+1].to_str().unwrap().split("/").collect::<Vec<&str>>();
    Ok(split_path[split_path.len()-1].to_string())
}

pub fn call_holochain(data: &models::HolochainUserRequst, pub_address: String) -> Result<models::HolochainResponse, errors::JuntoApiError>{
    let data = models::HolochainRequest{id: pub_address, jsonrpc: "2.0".to_string(), method: "call".to_string(), params: data.params.clone()};
    let request_data = serde_json::to_string(&data).map_err(|_err| errors::JuntoApiError::BadClientData)?;
    let client = reqwest::Client::new();
    let mut res = client.post(HC_CONDUCTOR)
                .header("context-type", "application/json")
                .body(request_data)
                .send()
                .map_err(|_err| errors::JuntoApiError::InternalError)?;
    let response_data: models::HolochainResponse = res.json().map_err(|_err| errors::JuntoApiError::InternalError)?;
    Ok(response_data)
}   
    //     let request_data = futures::future::result(serde_json::to_string(data)).map_err(|err| AWError::from(errors::JuntoApiError::InternalError));
    //     request_data.and_then(|request_data| {
    //         Client::default()
    //             .post(HC_CONDUCTOR) // <- Create request builder
    //             .header("Content-Type", "application/json")
    //             .send_body(&request_data)
    //             .map_err(AWError::from)
    //             .and_then(|response| {
    //                 //println!("Response: {:?}", response);
    //                 response
    //                     .from_err()
    //                     .fold(BytesMut::new(), |mut acc, chunk| {
    //                         acc.extend_from_slice(&chunk);
    //                         Ok::<_, AWError>(acc)
    //                     })
    //                     .map(|body| {
    //                         let response_data: models::HolochainResponse = serde_json::from_slice(&body).unwrap();
    //                         response_data
    //                     });
    //             })
    //     })
    // //validation.and_then(|_| post_response)