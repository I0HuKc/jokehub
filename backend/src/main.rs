mod db;
mod model;
mod server;
mod schema;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

use std::env;

#[launch]
fn rocket() -> _ {
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    db::run_migrations(&db_url);

    // let pool = db::get_pool(&db_url);

    server::launcher()
}
