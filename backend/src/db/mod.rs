pub mod errors;
pub mod joke_repository;

use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

use errors::{ERR_DB_CONN, ERR_DB_MIGRATION};

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

                    let conn = Conn::get_one(&r).await.expect(ERR_DB_CONN);
                    conn.run(|c| embedded_migrations::run(c))
                        .await
                        .expect(ERR_DB_MIGRATION);
                })
            }))
    }
}