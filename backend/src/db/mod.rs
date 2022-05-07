mod errors;
pub mod joke_repository;

use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;
use rocket::fairing::AdHoc;

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

                    let conn = Conn::get_one(&r).await.expect("database connection");
                    conn.run(|c| embedded_migrations::run(c))
                        .await
                        .expect("diesel migrations");
                })
            }))
    }
}
