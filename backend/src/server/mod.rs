mod anecdote_handler;
mod joke_handler;
pub mod ping_handler;
mod punch_handlers;

pub mod config;

use rocket::serde::json::{json, Value};

use crate::db::DbInit;

use anecdote_handler::*;
// use joke_handler::*;
use ping_handler::*;
use punch_handlers::*;

#[cfg(test)]
mod tests;

#[launch]
pub fn rocket() -> _ {
    rocket::custom(config::from_env())
        .manage_mongodb()
        .mount("/", rocket::routes![ping])
        .mount(
            "/v1",
            rocket::routes![
                // create_joke,
                create_anecdote,
                get_anecdote,
                create_punch,
                get_punch,
            ],
        )
        .register("/", catchers![not_found])
}

// impl Server {
//     fn new() -> Rocket<Build> {
//         rocket::custom(config::from_env())
//             .manage_mongodb()
//             .mount("/", rocket::routes![ping])
//             .mount(
//                 "/v1",
//                 rocket::routes![
//                     // create_joke,
//                     create_anecdote,
//                     get_anecdote,
//                     create_punch,
//                     get_punch,
//                 ],
//             )
//             .register("/", catchers![not_found])
//     }
// }

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
