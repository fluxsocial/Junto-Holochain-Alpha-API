use actix_web::{Error as AWError, error::ResponseError, HttpResponse, web};
use actix_identity::Identity;
use futures::future::Future;
use bcrypt::hash;

use crate::db;
use crate::errors;
use crate::utils;

pub fn register(data: web::Json<db::models::RegisterData>, pool: web::Data<db::Pool>) -> impl Future<Item = HttpResponse, Error = AWError>  {
    web::block(move || {
        let pub_key = utils::holochain::assign_agent(&pool)?;
        let id = uuid::Uuid::new_v4();
        let hashed_password = hash(data.password.clone(), 13).map_err(|_err| errors::JuntoApiError::InternalError)?;
        let user = db::models::Users{id: id.clone(), email: data.email.clone(), password: hashed_password, pub_address: pub_key, 
                    first_name: data.first_name.clone(), last_name: data.last_name.clone()};
        db::models::Users::insert_user(&user, &pool).map_err(|_err| errors::JuntoApiError::InternalError)?;
        let token = utils::jwt::Token::create(format!("{}", id)).map_err(|_err| errors::JuntoApiError::InternalError)?;
        Ok(token)
    })
    .then(|token: Result<String, actix_threadpool::BlockingError<errors::JuntoApiError>>| match token {
        Ok(token) => Ok(HttpResponse::Ok().json(json!({"token": token}))),
        Err(err) => match err {
                        actix_threadpool::BlockingError::Error(err) => {
                            Ok(err.error_response())
                        },
                        actix_threadpool::BlockingError::Canceled => {
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
        move |res: Result<db::models::SlimUser, actix_threadpool::BlockingError<errors::JuntoApiError>>| match res {
            Ok(user) => {
                let user_string = serde_json::to_string(&user).unwrap();
                id.remember(user_string);
                Ok(HttpResponse::Ok().finish())
            }
            Err(err) => match err {
                actix_threadpool::BlockingError::Error(service_error) => Err(service_error),
                actix_threadpool::BlockingError::Canceled => Err(errors::JuntoApiError::InternalError),
            },
        },
    )
}


pub fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}

pub fn get_me(logged_user: utils::middleware::LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}
