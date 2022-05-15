pub mod mongo;
pub mod sqlstore;

mod redis_connection;

use std::time::Duration;

use lazy_static::lazy_static;
use mongodb::{bson::doc, options::ClientOptions, sync::Client};
use rocket::{Build, Rocket};


pub trait DbInit {
    fn manage_mongodb(self) -> Self;   
}


impl DbInit for Rocket<Build> {
    fn manage_mongodb(self) -> Self {
        let client = connect_to_mongodb().unwrap();
        let mbox = Box::new(client);
        self.manage(mbox)
    }
}

fn connect_to_mongodb() -> Option<Client> {
    let mut options = ClientOptions::parse(dotenv!("MONGO_DB_URL")).unwrap();

    // Параметры соединения
    let duration: Duration = Duration::new(60, 0);
    options.app_name = Some("Stuffy Krill".to_string());
    options.connect_timeout = Some(duration);

    // Получение дескриптора кластера
    let client: Result<Client, mongodb::error::Error> = Client::with_options(options);
    match client {
        Ok(c) => {
            let ping = c
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .run_command(doc! {"ping": 1}, None)
                .unwrap();
            println!("{}", ping);
            Some(c)
        }
        Err(_) => Option::None,
    }
}

lazy_static! {
    static ref INFO_PG_CONN: &'static str = "Connect to PostgreSQL";
    static ref INFO_MONGO_CONN: &'static str = "Connect to MongoDB";
}

lazy_static! {
    static ref ERR_ENV_MONGO_URL: &'static str = "Unable to get MongoDB database url";
    static ref ERR_MONG_CONN: &'static str = "Cannot connect to MongoDB instance";
    static ref ERR_DB_CONN: &'static str = "Failed to establish a connection with DB";
    static ref ERR_DB_MIGRATION: &'static str = "Failed to roll migrations";
}

lazy_static! {
    pub static ref ERR_ENV_MONGO_DB_NAME: &'static str = "Unable to get MongoDB database name";
    pub static ref ERR_ALREADY_EXISTS: &'static str = "Record with these parameters already exists";
    pub static ref ERR_NOT_FOUND: &'static str = "Record with such parameters is not found";
    pub static ref ERR_INTERNAL: &'static str = "An database internal error has occurred";
}
