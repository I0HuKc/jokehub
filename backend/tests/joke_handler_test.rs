mod common;

use rocket::http::{Header, Status};

use common::{
    accounts::{TestMaster, TestPadawan},
    joke::TestNewJoke,
};

#[test]
fn create_joke() {
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match TestNewJoke::create_test_record(&client, Box::new(padawan)) {
        Ok((_, status, ..)) => assert_eq!(status, Status::Ok),
        Err(err) => assert!(false, "\n\nFaild to create test record: {}\n\n", err),
    }
}

#[test]
fn get_punch() {
    let path: &str = "/v1/joke/";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match TestNewJoke::create_test_record(&client, Box::new(padawan)) {
        Ok((_, status, id)) => {
            assert_eq!(status, Status::Ok);

            // Получение записи
            {
                let resp = client.get(format!("{}/{}", path, id)).dispatch();
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

        Err(err) => assert!(false, "\n\nFaild to create test record: {}\n\n", err),
    }
}

#[test]
fn delete_punch_by_padawan() {
    let path: &str = "/v1/joke/";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match TestNewJoke::create_test_record(&client, Box::new(padawan)) {
        Ok((tokens, status, id)) => {
            assert_eq!(status, Status::Ok);

            let resp = client
                .delete(format!("{}/{}", path, id))
                .header(bearer!((tokens.access_token)))
                .dispatch();

            assert_eq!(resp.status(), Status::Forbidden);
        }
        Err(err) => assert!(false, "\n\nFaild to create test record: {}\n\n", err),
    }
}

#[test]
fn delete_punch_by_master() {
    let path: &str = "/v1/joke/";
    let client = common::test_client().lock().unwrap();
    let master = TestMaster::default();

    match TestNewJoke::create_test_record(&client, Box::new(master)) {
        Ok((tokens, status, id)) => {
            assert_eq!(status, Status::Ok);

            let resp = client
                .delete(format!("{}/{}", path, id))
                .header(bearer!((tokens.access_token)))
                .dispatch();

            assert_eq!(resp.status(), Status::Ok);
        }
        Err(err) => assert!(false, "\n\nFaild to create test record: {}\n\n", err),
    }
}
