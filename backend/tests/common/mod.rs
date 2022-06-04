pub mod accounts;
pub mod anecdote;
pub mod joke;
pub mod punch;

use once_cell::sync::OnceCell;
use rocket::local::blocking::{Client, LocalResponse};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Mutex;

use jokehub::server;

#[derive(Deserialize, Debug)]
pub struct RegResp {
    #[allow(dead_code)]
    id: String,
}

#[macro_export]
macro_rules! bearer {
    ($at:expr) => {
        Header::new("Authorization", format!("Bearer {}", $at))
    };
}

#[macro_export]
macro_rules! apikey {
    ($key:literal) => {
        Header::new("Api-Key", $key)
    };
}

#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
    };
}

#[macro_export]
macro_rules! assert_body {
    ( $resp:expr, $t:tt$(<$g:path>)?) => {{
        use serde_json::Value;

        let body = $resp.into_string().unwrap();
        let value: Value = serde_json::from_str(&body).expect("can't parse value");

        serde_json::from_str::<$t>(value.to_string().as_str())
            .expect(format!("{} valid response", stringify!($t)).as_str())
    }};
}

#[allow(dead_code)]
pub fn test_client() -> &'static Mutex<Client> {
    static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();

    INSTANCE.get_or_init(|| {
        let server = server::rocket();
        Mutex::from(Client::tracked(server).expect("invalid rocket instance"))
    })
}

#[allow(dead_code)]
pub fn response_json_value<'a>(response: LocalResponse<'a>) -> Value {
    let body = response.into_string().unwrap();
    serde_json::from_str(&body).expect("can't parse value")
}
