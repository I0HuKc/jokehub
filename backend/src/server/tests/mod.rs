mod ping_handler;

use once_cell::sync::OnceCell;
use rocket::local::blocking::Client;
use std::sync::Mutex;

use crate::server;

pub(crate) fn test_client() -> &'static Mutex<Client> {
    static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let server = server::rocket();
        Mutex::from(Client::tracked(server).expect("valid rocket instance"))
    })
}
