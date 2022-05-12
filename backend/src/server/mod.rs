mod anecdote_handler;
mod joke_handler;
mod punch_handlers;

use rocket::{Build, Rocket};

use crate::db::DbInit;

use anecdote_handler::*;
use joke_handler::*;
use punch_handlers::*;


pub trait Server {
    fn launch(self) -> Self;
}

impl Server for Rocket<Build> {
    fn launch(self) -> Self {
        self
            .manage_postgres()
            .manage_mongodb()
            .mount(
                "/v1",
                rocket::routes![
                    create_joke, 

                    create_anecdote, 
                    get_anecdote,

                    create_punch,
                    get_punch,
                ],
        )
    }
}
