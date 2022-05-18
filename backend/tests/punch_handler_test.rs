use common::{accounts::TestPadawan, punch::TestNewPunch};
use rocket::http::{ContentType, Header, Status};

mod common;

#[test]
fn get_punch() {
    let path: &str = "/v1/punch/";
    let client = common::test_client().lock().unwrap();

    let padawan = TestPadawan::default();
    let punch = TestNewPunch::default();

    // Создаю запись
    match common::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            // Создаю запись
            let header = Header::new("Authorization", format!("Bearer {}", tokens.access_token));

            let resp = client
                .post(format!("{}/new", path))
                .header(header)
                .header(ContentType::JSON)
                .body(json_string!({
                    "setup": punch.setup,
                    "punchline": punch.punchline,
                    "language": "ru"
                }))
                .dispatch();

            assert_eq!(resp.status(), Status::Ok);

            let value = common::response_json_value(resp);

            let pid = value
                .get("id")
                .expect("must have a 'id' field")
                .as_str()
                .expect("must have 'id' value in str format");

            // Получение записи
            {
                let resp = client.get(format!("{}/{}", path, pid)).dispatch();
                assert_eq!(resp.status(), Status::Ok);
            }

            // Получение по неправильному uuid
            {
                let resp = client.get(format!("{}/invalid-format", path)).dispatch();
                assert_eq!(resp.status(), Status::UnprocessableEntity); 
            }

            // Получение не существующей записи
            {
                let resp = client.get(format!("{}/fe16b7b2-54cc-45d0-8162-7819f463f5d4", path)).dispatch();
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

    match common::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            let header = Header::new("Authorization", format!("Bearer {}", tokens.access_token));

            let resp = client
                .post(format!("{}", path))
                .header(header)
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
