use crate::errors::Errors;
use serde_json::{json, Value};

#[get("/ping")]
pub async fn ping<'f>() -> Result<Value, Errors<'f>> {
    Ok(json!({"ping": "pong"}))
}
