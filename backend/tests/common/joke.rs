use rand::{distributions::Alphanumeric, Rng};
use rocket::{
    http::{ContentType, Header, Status},
    local::blocking::Client,
};
use serde_json::Value;
use std::sync::MutexGuard;

use super::{accounts as account, accounts::TestUser, RegResp};
use jokehub::model::account::security::Tokens;

pub struct TestNewJoke {
    pub text: String,
}

impl TestNewJoke {
    #[allow(dead_code)]
    pub fn create_test_record(
        client: &MutexGuard<Client>,
        user: Box<dyn TestUser>,
    ) -> Result<(Tokens, Status, String), Value> {
        let path: &str = "/v1/joke/new";
        let joke = TestNewJoke::default();

        match account::try_login(&client, user) {
            Ok(tokens) => {
                let resp = client
                    .post(format!("{}", path))
                    .header(crate::bearer!((tokens.access_token)))
                    .header(ContentType::JSON)
                    .body(crate::json_string!({
                        "text": joke.text,
                        "language": "ru"
                    }))
                    .dispatch();

                Ok((tokens, resp.status(), crate::assert_body!(resp, RegResp).id))
            }

            Err(err) => Err(err),
        }
    }
}

impl Default for TestNewJoke {
    fn default() -> Self {
        let salt: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();

        Self {
            text: format!("Как каннибал называет Пашу? {}", salt),
        }
    }
}
