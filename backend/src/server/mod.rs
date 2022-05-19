mod account_handlers;
mod anecdote_handler;
mod config;
mod joke_handler;
mod ping_handler;
mod punch_handler;

use crate::{
    db::DbManage,
    err_not_found,
    errors::{ErrorKind, HubError, UnauthorizedErrorKind},
};

use {
    // use joke_handler::*;
    account_handlers::*,
    anecdote_handler::*,
    ping_handler::*,
    punch_handler::*,
};

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
                delete_punch,
                registration,
                login,
                account,
                refresh_token,
                logout,
                delete_account,
            ],
        )
        .register("/", catchers![not_found, unauthorized, internal])
}

#[catch(404)]
fn not_found() -> HubError {
    err_not_found!("page", "some det", "dfddg")
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
