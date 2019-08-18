use actix_web::{error, http, HttpResponse};
use failure::Fail;

#[derive(Fail, Debug)]
pub enum JuntoApiError {
    #[fail(display = "Internal Error Has Occured")]
    InternalError,
    #[fail(display = "Bad Request Data")]
    BadClientData,
    #[fail(display = "Request Timeout")]
    Timeout,
}

impl error::ResponseError for JuntoApiError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            JuntoApiError::InternalError => {
                HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            JuntoApiError::BadClientData => HttpResponse::new(http::StatusCode::BAD_REQUEST),
            JuntoApiError::Timeout => HttpResponse::new(http::StatusCode::GATEWAY_TIMEOUT),
        }
    }
}