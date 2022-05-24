pub mod mongo;

use rocket::{Build, Rocket};

pub trait DbManage {
    fn manage_mongodb(self) -> Self;
}

impl DbManage for Rocket<Build> {
    fn manage_mongodb(self) -> Self {
        let client = mongo::connect().unwrap();
        let mbox = Box::new(client);
        self.manage(mbox)
    }
}
