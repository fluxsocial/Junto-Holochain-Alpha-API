use actix_web::{error::ResponseError, HttpResponse, web};
use actix_identity::Identity;
use actix_threadpool::BlockingError;
use futures::future::Future;
use bcrypt::{hash, DEFAULT_COST};

use crate::db;
use crate::errors;
use crate::utils;

pub fn register(
    data: web::Json<db::models::RegisterData>, 
    pool: web::Data<db::Pool>
) -> impl Future<Item = HttpResponse, Error = errors::JuntoApiError> {
    web::block(move || {
        let pub_key = utils::holochain::assign_agent(&pool)?;
        let user_id = uuid::Uuid::new_v4();
        let hashed_password = hash(data.password.clone(), DEFAULT_COST).map_err(|_err| errors::JuntoApiError::InternalError)?;
        let user = db::models::Users{id: user_id.clone(), email: data.email.clone(), password: hashed_password, pub_address: pub_key, 
                    first_name: data.first_name.clone(), last_name: data.last_name.clone()};
        db::models::Users::insert_user(&user, &pool).map_err(|_err| errors::JuntoApiError::InternalError)?;
        Ok(user_id.to_string())
    })
    .then(|user_id: Result<String, BlockingError<errors::JuntoApiError>>| match user_id {
        Ok(user_id) => Ok(HttpResponse::Ok().json(json!({"user_id": user_id}))),
        Err(err) => match err {
                        BlockingError::Error(err) => {
                            Ok(err.error_response())
                        },
                        BlockingError::Canceled => {
                            Ok(HttpResponse::InternalServerError().into())
                        }
                    },
    })
}

pub fn login(
    auth_data: web::Json<db::models::AuthData>,
    id: Identity,
    pool: web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = errors::JuntoApiError> {
    web::block(move || db::models::Users::can_login(auth_data.into_inner(), &pool)).then(
        move |res: Result<db::models::SlimUser, BlockingError<errors::JuntoApiError>>| match res {
            Ok(user) => {
                let user_string = serde_json::to_string(&user).unwrap();
                id.remember(user_string);
                Ok(HttpResponse::Ok().finish())
            }
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(errors::JuntoApiError::InternalError),
            },
        },
    )
}


pub fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}