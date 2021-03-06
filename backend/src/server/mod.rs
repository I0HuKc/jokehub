mod account_handler;
mod anecdote_handler;
mod base_handler;
mod favorite_handler;
mod joke_handler;
mod punch_handler;
mod shrimp_handler;

mod config;
mod lingua;

use crate::db::DbManage;

use self::lingua::LinguaManage;

use {
    account_handler::*, anecdote_handler::*, base_handler::*, favorite_handler::*, joke_handler::*,
    punch_handler::*, shrimp_handler::*,
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
                // Anecdote methods
                create_anecdote,
                get_anecdote,
                delete_anecdote,
                reaction_anecdote,
                // Punch methods
                create_punch,
                get_punch,
                delete_punch,
                reaction_punch,
                // Jokes methods
                create_joke,
                get_joke,
                delete_joke,
                reaction_joke,
                // Shrimp methods
                random,
                // Accounts methods
                password_strength,
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
                // Api-Key methods
                new_api_key,
                del_api_key,
                // Favorite methods
                favorite_add,
                favorite_remove
            ],
        )
        .register("/", catchers![not_found, unauthorized, internal, forbidden])
}
