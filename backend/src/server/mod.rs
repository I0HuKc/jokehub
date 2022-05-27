mod account_handler;
mod anecdote_handler;
mod base_handler;
mod favorite_handler;
mod joke_handler;
mod punch_handler;
mod random_handler;

mod config;
mod lingua;

use crate::db::DbManage;

use self::lingua::LinguaManage;

use {
    account_handler::*, anecdote_handler::*, base_handler::*, favorite_handler::*, joke_handler::*,
    punch_handler::*, random_handler::*,
};

#[launch]
pub fn rocket() -> _ {
    rocket::custom(config::from_env())
        .manage_mongodb()
        .manage_lingua()
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
                // Random
                random,
                // Accounts methods
                registration,
                login,
                account,
                refresh_token,
                change_password,
                change_theme,
                logout,
                logout_any,
                delete_account,
                privilege,
                // Favorite
                favorite_add
            ],
        )
        .register("/", catchers![not_found, unauthorized, internal, forbidden])
}
