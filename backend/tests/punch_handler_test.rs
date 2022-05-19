use common::{accounts::TestPadawan, punch::TestNewPunch};
use rocket::http::{ContentType, Header, Status};
use serde_json::Value;

mod common;

use common::accounts as account;
use serde::Deserialize;

#[test]
fn get_punch() {
    let path: &str = "/v1/punch/";
    let client = common::test_client().lock().unwrap();

    let padawan = TestPadawan::default();
    let punch = TestNewPunch::default();

    // Создаю запись
    match account::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            #[derive(Deserialize, Debug)]
            struct RegResp {
                #[allow(dead_code)]
                id: String,
            }

            // Создаю запись
            let resp = client
                .post(format!("{}/new", path))
                .header(bearer!((tokens.access_token)))
                .header(ContentType::JSON)
                .body(json_string!({
                    "setup": punch.setup,
                    "punchline": punch.punchline,
                    "language": "ru"
                }))
                .dispatch();

            assert_eq!(resp.status(), Status::Ok);

            // Получение записи
            {
                let resp = client
                    .get(format!("{}/{}", path, assert_body!(resp, RegResp).id))
                    .dispatch();
                assert_eq!(resp.status(), Status::Ok);
            }

            // Получение по неправильному uuid
            {
                let resp = client.get(format!("{}/invalid-format", path)).dispatch();
                assert_eq!(resp.status(), Status::UnprocessableEntity);
            }

            // Получение не существующей записи
            {
                let resp = client
                    .get(format!("{}/fe16b7b2-54cc-45d0-8162-7819f463f5d4", path))
                    .dispatch();
                assert_eq!(resp.status(), Status::NotFound);
            }
        }

        Err(err) => assert!(false, "\n\nFaild to login: {}\n\n", err),
    };
}

#[test]
fn create_punch() {
    let path: &str = "/v1/punch/new";
    let client = common::test_client().lock().unwrap();

    let padawan = TestPadawan::default();
    let punch = TestNewPunch::default();

    match account::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            let resp = client
                .post(format!("{}", path))
                .header(bearer!((tokens.access_token)))
                .header(ContentType::JSON)
                .body(json_string!({
                    "setup": punch.setup,
                    "punchline": punch.punchline,
                    "language": "ru"
                }))
                .dispatch();

            assert_eq!(resp.status(), Status::Ok)
        }

        Err(err) => assert!(false, "\n\nFaild to login: {}\n\n", err),
    }
}
