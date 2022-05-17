mod account_handlers;
mod anecdote_handler;
mod config;
mod joke_handler;
mod ping_handler;
mod punch_handler;

use crate::{
    db::DbManage,
    errors::{ErrorKind, HubError, UnauthorizedErrorKind, message::ERR_NOT_FOUND},
};

use {
    // use joke_handler::*;
    account_handlers::*,
    anecdote_handler::*,
    ping_handler::*,
    punch_handler::*,
};

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
        .register("/", catchers![not_found, unauthorized, internal])
}

#[catch(404)]
fn not_found() -> HubError {
    HubError::new_not_found(ERR_NOT_FOUND.as_ref(), None)
}

#[catch(500)]
fn internal() -> HubError {
    HubError::new_internal("Opps, something went wrong...", None)
}

#[catch(401)]
fn unauthorized() -> HubError {
    let kind = ErrorKind::Unauthorized(UnauthorizedErrorKind::Generic("Authorization required"));
    HubError::new(kind)
}
