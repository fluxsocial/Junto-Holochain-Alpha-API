use std::fs;
use actix_web::web;
use diesel::result::Error;

use crate::db::{
    models,
    Pool 
};

static KEY_DIR: &'static str = "/home/josh/.config/holochain/keys/";

pub fn assign_agent(pool: web::Data<Pool>) -> Result<String, Error> {
    let possible_keys: Vec<_> = fs::read_dir(KEY_DIR).unwrap().map(|res| res.unwrap().path()).collect();
    let n_users = models::Users::number_of_users(pool)?;
    let split_path = possible_keys[n_users+1].to_str().unwrap().split("/").collect::<Vec<&str>>();
    Ok(split_path[split_path.len()-1].to_string())
}