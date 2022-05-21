mod common;

use common::{accounts::{TestPadawan, TestSith}, anecdote::TestNewAnecdote};
use rocket::http::{Header, Status};

#[test]
fn create_anecdote() {
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match TestNewAnecdote::create_test_record(&client, Box::new(padawan)) {
        Ok((_, status, ..)) => assert_eq!(status, Status::Ok),
        Err(err) => assert!(false, "\n\nFaild to create test record: {}\n\n", err),
    }
}

#[test]
fn get_anecdote() {
    let path: &str = "/v1/anecdote/";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match TestNewAnecdote::create_test_record(&client, Box::new(padawan)) {
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
fn delete_anecdote_by_padawan() {
    let path: &str = "/v1/anecdote/";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match TestNewAnecdote::create_test_record(&client, Box::new(padawan)) {
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
fn delete_anecdote_by_sith() {
    let path: &str = "/v1/anecdote/";
    let client = common::test_client().lock().unwrap();
    let sith = TestSith::default();

    match TestNewAnecdote::create_test_record(&client, Box::new(sith)) {
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

