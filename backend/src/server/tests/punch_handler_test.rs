use rocket::http::{ContentType, Status};

use super::common::*;
use crate::json_string;

#[test]
fn get_punch() {
    let client = test_client().lock().unwrap();

    // Создание тестовой записи
    let resp = client
        .post("/v1/punch/new")
        .header(ContentType::JSON)
        .body(json_string!({
            "language": "ru",
            "setup": "setup text",
            "punchline": "punchline text"
        }))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let value = response_json_value(resp);
    let id = value
        .get("id")
        .expect("must have a 'id' field")
        .as_str()
        .expect("must have 'id' value in str format");

    println!("{}", id);

    // Получение записи
    let test_cases: Vec<(&str, &str, Status)> = vec![
        (
            "invalid fotmat_1",
            "some_invalid_id",
            Status::UnprocessableEntity,
        ),
        (
            "invalid fotmat_2",
            "b7b24959-3aa3-461a-a01a-c805697deeb",
            Status::UnprocessableEntity,
        ),
        (
            "valid", 
            id,
            Status::Ok
        ),
    ];

    for tc in test_cases {
        let resp = client
            .get(format!("/v1/punch/{}", tc.1))
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(resp.status(), tc.2, "{}", tc.0);

        if resp.status() != tc.2 {
            println!("{:?}", response_json_value(resp));
        }
    }
}

#[test]
fn create_punch() {
    let client = test_client().lock().unwrap();

    let test_cases: Vec<TestCase> = vec![
        // Invalid language [type]
        TestCase(
            json_string!({
                "language": "es",
                "setup": "Как каннибал называет Пашу?",
                "punchline": "Паштет"
            }),
            Status::UnprocessableEntity,
        ),
        // Invalid language [lenght]
        TestCase(
            json_string!({
                "language": "ruu",
                "setup": "Как каннибал называет Пашу?",
                "punchline": "Паштет"
            }),
            Status::UnprocessableEntity,
        ),
        // Valid with empty tags
        TestCase(
            json_string!({
                "language": "ru",
                "setup": "Как каннибал называет Пашу?",
                "punchline": "Паштет"
            }),
            Status::Ok,
        ),
        // Valid
        TestCase(
            json_string!({
                "tags": ["meme"],
                "language": "ru",
                "setup": "Знаешь какую статью присудили карлику?",
                "punchline": "Мелкое хулиганство"
            }),
            Status::Ok,
        ),
    ];

    for tc in test_cases {
        let resp = client
            .post("/v1/punch/new")
            .header(ContentType::JSON)
            .body(tc.0)
            .dispatch();

        assert_eq!(resp.status(), tc.1);

        if resp.status() != tc.1 {
            println!("{:?}", response_json_value(resp));
        }
    }
}
