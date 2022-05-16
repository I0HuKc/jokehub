use crate::errors::HubError;
use serde_json::{json, Value};

#[get("/ping")]
pub async fn ping<'f>() -> Result<Value, HubError> {
    Ok(json!({"ping": "pong"}))
}
