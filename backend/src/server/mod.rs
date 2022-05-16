mod account_handlers;
mod anecdote_handler;
mod config;
mod joke_handler;
mod ping_handler;
mod punch_handler;

use rocket::serde::json::{json, Value};

use crate::db::DbManage;

use anecdote_handler::*;
// use joke_handler::*;
use account_handlers::*;
use ping_handler::*;
use punch_handler::*;

#[cfg(test)]
mod tests;

#[launch]
pub fn rocket() -> _ {
    rocket::custom(config::from_env())
        .manage_mongodb()
        .manage_redis()
        .mount("/", rocket::routes![ping])
        .mount(
            "/v1",
            rocket::routes![
                // create_joke,
                create_anecdote,
                get_anecdote,
                create_punch,
                get_punch,
                registration,
                get_user,
                login
            ],
        )
        .register("/", catchers![not_found])
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
