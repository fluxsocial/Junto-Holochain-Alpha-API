use actix_web::{HttpRequest, dev::Payload, FromRequest};
use actix_identity::Identity;

use crate::models;
use crate::errors;

pub type LoggedUser = models::db::user::SlimUser;

impl FromRequest for LoggedUser {
    type Config = ();
    type Error = errors::JuntoApiError;
    type Future = Result<LoggedUser, errors::JuntoApiError>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Some(identity) = Identity::from_request(req, pl).map_err(|_err| errors::JuntoApiError::BadClientData)?.identity() {
            let user: LoggedUser = serde_json::from_str(&identity).map_err(|_err| errors::JuntoApiError::BadClientData)?;
            return Ok(user);
        }
        Err(errors::JuntoApiError::Unauthorized.into())
    }
}
