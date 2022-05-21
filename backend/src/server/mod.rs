mod account_handler;
mod anecdote_handler;
mod config;
mod joke_handler;
mod base_handler;
mod punch_handler;

use crate::{
    db::DbManage, err_forbidden, err_internal, err_not_found, err_unauthorized, errors::HubError,
};

use {
    account_handler::*, anecdote_handler::*, joke_handler::*, base_handler::*, punch_handler::*,
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
                // Punch methods
                create_anecdote,
                get_anecdote,
                delete_anecdote,
                // Punch methods
                create_punch,
                get_punch,
                delete_punch,
                // Jokes methods
                create_joke,
                get_joke,
                delete_joke,
                // Accounts methods
                registration,
                login,
                account,
                refresh_token,
                logout,
                delete_account,
                privilege,
            ],
        )
        .register("/", catchers![not_found, unauthorized, internal, forbidden])
}

#[catch(401)]
fn unauthorized() -> HubError {
    err_unauthorized!("Authorization required")
}

#[catch(403)]
fn forbidden() -> HubError {
    err_forbidden!()
}

#[catch(404)]
fn not_found() -> HubError {
    err_not_found!("page")
}

#[catch(500)]
fn internal() -> HubError {
    err_internal!("Opps, something went wrong...")
}
