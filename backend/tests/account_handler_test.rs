use rocket::http::{ContentType, Status};
use test_case::test_case;
use uuid::Uuid;

mod common;

#[test_case(
    json_string!({
        "username": "I0H uKc",
        "password": "12344321e"
    }),
    Status::UnprocessableEntity ;
    "invalid username format"
)]
#[test_case(
    json_string!({
        "username": "I0",
        "password": "12344321e"
    }),
    Status::UnprocessableEntity ;
    "invalid username lenght [min]"
)]
#[test_case(
    json_string!({
        "username": "1234567890123456",
        "password": "12344321e"
    }),
    Status::UnprocessableEntity ;
    "invalid username lenght [max]"
)]
#[test_case(
    json_string!({
        "username": "I0HuKc",
        "password": "1234567"
    }),
    Status::UnprocessableEntity ;
    "invalid password lenght [min]"
)]
#[test_case(
    json_string!({
        "username": "I0HuKc",
        "password": "123456789012345678901234567890"
    }),
    Status::UnprocessableEntity ;
    "invalid password lenght [max]"
)]
fn login(body: String, status: Status) {
    let path: &str = "/v1/login";
    let client = common::test_client().lock().unwrap();

    let resp = client
        .post(path)
        .header(ContentType::JSON)
        .body(body)
        .dispatch();

    let resp_status = resp.status();
    let value = common::response_json_value(resp);

    assert_eq!(resp_status, status);

    if resp_status == Status::Ok {
        let access_token = value
            .get("access_token")
            .expect("must have an 'access_token' field")
            .as_str();
        match access_token {
            Some(token) => assert_ne!("", token),
            None => assert!(false),
        }

        let refresh_token = value
            .get("refresh_token")
            .expect("must have an 'refresh_token' field")
            .as_str();
        match refresh_token {
            Some(token) => assert_ne!("", token),
            None => assert!(false),
        }
    }
}

#[test_case(
    json_string!({
        "username": "I0H uKc",
        "password": "12344321e"
    }),
    Status::UnprocessableEntity ;
    "invalid username format"
)]
#[test_case(
    json_string!({
        "username": "I0",
        "password": "12344321e"
    }),
    Status::UnprocessableEntity ;
    "invalid username lenght [min]"
)]
#[test_case(
    json_string!({
        "username": "3851d279-baa6-4f4c-8ace-c9d472ef2c5735ее56546",
        "password": "12344321e"
    }),
    Status::UnprocessableEntity ;
    "invalid username lenght [max]"
)]
#[test_case(
    json_string!({
        "username": "I0HuKc",
        "password": "1234567"
    }),
    Status::UnprocessableEntity ;
    "invalid password lenght [min]"
)]
#[test_case(
    json_string!({
        "username": "I0HuKc",
        "password": "123456789012345678901234567890"
    }),
    Status::UnprocessableEntity ;
    "invalid password lenght [max]"
)]
#[test_case(
    json_string!({
        "username": "I0HuKc",
        "password": "12344321e"
    }),
    Status::Ok ;
    "valid"
)]
fn registration(body: String, status: Status) {
    let path: &str = "/v1/registration";
    let client = common::test_client().lock().unwrap();

    let resp = client
        .post(path)
        .header(ContentType::JSON)
        .body(body)
        .dispatch();

    let resp_status = resp.status();
    let value = common::response_json_value(resp);

    assert_eq!(resp_status, status);

    if resp_status == Status::Ok {
        let user_id = value.get("id").expect("must have an 'id' field").as_str();
        match user_id {
            Some(id) => match Uuid::parse_str(id) {
                Ok(_) => assert!(true),
                Err(_) => assert!(false),
            },
            None => assert!(false),
        }
    }
}

mod auth_guard {
    use crate::common;
    use rocket::http::{Header, Status};

    #[test]
    fn invalid_format_without_bearer() {
        let path: &str = "/v1/account";
        let client = common::test_client().lock().unwrap();

        match common::try_login(&client) {
            Ok(tokens) => {
                let header =
                    Header::new("Authorization", format!("{}", tokens.access_token));

                let resp = client
                    .get(format!("{}", path))
                    .header(header)
                    .dispatch();

                assert_eq!(resp.status(), Status::Unauthorized);
            }
            Err(err) => {
                assert!(false, "\n\nFaild to login: {}\n\n", err)
            }
        }
    }

    #[test]
    fn invalid_format_without_token() {
        let path: &str = "/v1/account";
        let client = common::test_client().lock().unwrap();

        match common::try_login(&client) {
            Ok(_) => {
                let header = Header::new("Authorization", format!("Bearer"));

                let resp = client
                    .get(format!("{}", path))
                    .header(header)
                    .dispatch();

                assert_eq!(resp.status(), Status::Unauthorized);
            }
            Err(err) => {
                assert!(false, "\n\nFaild to login: {}\n\n", err)
            }
        }
    }

    #[test]
    fn without_header() {
        let path: &str = "/v1/account";
        let client = common::test_client().lock().unwrap();

        match common::try_login(&client) {
            Ok(_) => {
                let resp = client.get(format!("{}", path)).dispatch();

                assert_eq!(resp.status(), Status::Unauthorized);
            }
            Err(err) => {
                assert!(false, "\n\nFaild to login: {}\n\n", err)
            }
        }
    }

    #[test]
    fn valid() {
        let path: &str = "/v1/account";
        let client = common::test_client().lock().unwrap();

        match common::try_login(&client) {
            Ok(tokens) => {
                let header = Header::new(
                    "Authorization",
                    format!("Bearer {}", tokens.access_token),
                );

                let resp = client
                    .get(format!("{}", path))
                    .header(header)
                    .dispatch();

                assert_eq!(resp.status(), Status::Ok);
            }
            Err(err) => {
                assert!(false, "\n\nFaild to login: {}\n\n", err)
            }
        }
    }
}
