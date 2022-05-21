use crate::{err_forbidden, err_internal, err_not_found, err_unauthorized, errors::HubError};
use serde_json::{json, Value};

#[catch(401)]
pub fn unauthorized() -> HubError {
    err_unauthorized!("Authorization required")
}

#[catch(403)]
pub fn forbidden() -> HubError {
    err_forbidden!()
}

#[catch(404)]
pub fn not_found() -> HubError {
    err_not_found!("page")
}

#[catch(500)]
pub fn internal() -> HubError {
    err_internal!("Opps, something went wrong...")
}

#[get("/ping")]
pub fn ping<'f>() -> Result<Value, HubError> {
    Ok(json!({"ping": "pong"}))
}
