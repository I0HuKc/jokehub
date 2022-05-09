pub mod joke_repository;

use lazy_static::lazy_static;
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

pub trait DbInit {
    fn manage_db(self) -> Self;
}

#[database("jokehub_db")]
pub struct Conn(diesel::PgConnection);

impl DbInit for Rocket<Build> {
    fn manage_db(self) -> Self {
        self.attach(Conn::fairing())
            .attach(AdHoc::on_liftoff("", |r| {
                Box::pin(async move {
                    embed_migrations!();

                    let conn = Conn::get_one(&r).await.expect(ERR_DB_CONN.clone());
                    conn.run(|c| embedded_migrations::run(c))
                        .await
                        .expect(ERR_DB_MIGRATION.clone());
                })
            }))
    }
}

lazy_static! {
    static ref ERR_DB_CONN: &'static str = "Failed to establish a connection with DB";
    static ref ERR_DB_MIGRATION: &'static str = "Failed to roll migrations";
}

lazy_static! {
    pub static ref ERR_ALREADY_EXISTS: &'static str = "Record with these parameters already exists";
    pub static ref ERR_NOT_FOUND: &'static str = "Record with such parameters is not found";
}
