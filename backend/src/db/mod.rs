pub mod mongo;
pub mod redis;

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

