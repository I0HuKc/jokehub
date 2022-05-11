mod anecdote_handler;
mod joke_handler;

use rocket::{Build, Rocket};

use crate::db::DbInit;

use anecdote_handler::*;
use joke_handler::*;

pub trait Server {
    fn launch(self) -> Self;
}

impl Server for Rocket<Build> {
    fn launch(self) -> Self {
        self.manage_postgres().manage_mongodb().mount(
            "/v1",
            rocket::routes![create_joke, create_anecdote, get_anecdote],
        )
    }
}
