use std::fs;

use crate::db::{models, Pool };
use crate::errors;

static KEY_DIR: &'static str = "/Users/Josh/.config/holochain/keys";

pub fn assign_agent(pool: &Pool) -> Result<String, errors::JuntoApiError> {
    let possible_keys: Vec<_> = fs::read_dir(KEY_DIR).unwrap().map(|res| res.unwrap().path()).collect();
    let n_users = models::Users::number_of_users(pool).map_err(|_err| errors::JuntoApiError::InternalError)?;
    println!("Number of users: {}", n_users);
    let split_path = possible_keys[n_users+1].to_str().unwrap().split("/").collect::<Vec<&str>>();
    Ok(split_path[split_path.len()-1].to_string())
}