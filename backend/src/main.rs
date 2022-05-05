mod model;
mod server;

#[macro_use]
extern crate rocket;

extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::env;

use diesel::{
    connection::Connection,
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};


#[launch]
fn rocket() -> _ {
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    run_migrations(&db_url);

    server::launcher()
}

pub fn run_migrations(db_url: &str) {
    embed_migrations!();
    let connection = PgConnection::establish(db_url).expect("Error connecting to database");
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Error running migrations");
}

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}
