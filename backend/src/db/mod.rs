pub mod mongo;
pub mod redis;

use lazy_static::lazy_static;
use mongodb::bson::doc;
use rocket::{Build, Rocket};

pub trait DbManage {
    fn manage_mongodb(self) -> Self;
    fn manage_redis(self) -> Self;
}

impl DbManage for Rocket<Build> {
    fn manage_mongodb(self) -> Self {
        let client = mongo::connect().unwrap();
        let mbox = Box::new(client);
        self.manage(mbox)
    }

    fn manage_redis(self) -> Self {
        self.manage(redis::connect())
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
