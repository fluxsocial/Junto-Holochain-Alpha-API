use actix_web::{
    dev::Payload, Error as AWError, FromRequest, HttpRequest
};
use actix_identity::Identity;

use crate::errors::JuntoApiError;
use crate::db::models::SlimUser;

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;

// to get LoggedUser from auth cookie
impl FromRequest for LoggedUser {
    type Error = AWError;
    type Future = Result<LoggedUser, AWError>;
    type Config = ();

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Some(identity) = Identity::from_request(req, pl)?.identity() {
            let user: LoggedUser = serde_json::from_str(&identity)?;
            return Ok(user);
        }
        Err(JuntoApiError::Unauthorized.into())
    }
}