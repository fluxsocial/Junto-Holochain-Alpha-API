use actix_web::{error::ResponseError, web, HttpResponse};
use actix_threadpool::BlockingError;
use futures::future::Future;

use crate::errors;
use crate::db;
use crate::utils;

pub fn holochain(data: web::Json<db::models::HolochainUserRequst>, pool: web::Data<db::Pool>, logged_user: utils::wrapper::LoggedUser) 
    -> impl Future<Item = HttpResponse, Error = errors::JuntoApiError> {
    web::block(move ||{
        println!("User ID: {:?}", logged_user.id);
        let pub_address = db::models::Users::get_pub_key(&logged_user.id, &pool)?;
        println!("Holochain request using pub key: {:?}", pub_address);
        let holochain_call = utils::holochain::call_holochain(data.into_inner(), pub_address)?;
        Ok(holochain_call)
    })
    .then(|holochain_result: Result<db::models::HolochainResponse, BlockingError<errors::JuntoApiError>>| 
        match holochain_result {
            Ok(holochain_result) => Ok(HttpResponse::Ok().json(holochain_result)),
            Err(err) => match err {
                            BlockingError::Error(err) => {
                                Ok(err.error_response())
                            },
                            BlockingError::Canceled => {
                                Ok(HttpResponse::InternalServerError().into())
                            }
                        }
        }
    )
}