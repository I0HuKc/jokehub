mod common;

use rocket::http::{ContentType, Header, Status};
use rocket::local::blocking::Client;
use uuid::Uuid;

use common::{accounts::TestPadawan, response_json_value};
use jokehub::model::account::{
    security::{AccessClaims, RefreshClaims, Tokens},
    Tariff, UserResp,
};

#[test]
fn login() {
    let path: &str = "/v1/login";
    let client = common::test_client().lock().unwrap();

    let resp = client
        .post(format!("{}", path))
        .header(ContentType::JSON)
        .body(json_string!({
            "username": "I0HuKc",
            "password": "1234password"
        }))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let value = response_json_value(resp).to_string();
    let _: Tokens = serde_json::from_str(&value.as_str()).expect("login valid response");
}

#[test]
fn registration() {
    let path: &str = "/v1/registration";
    let client = common::test_client().lock().unwrap();

    let resp = client
        .post(path)
        .header(ContentType::JSON)
        .body(json_string!({
            "username": "I0HuKc",
            "password": "1234password"
        }))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let value = common::response_json_value(resp);
    let user_id = value.get("id").expect("must have an 'id' field").as_str();

    match user_id {
        Some(id) => match Uuid::parse_str(id) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        },
        None => assert!(false),
    }
}

#[test]
fn account_padawan() {
    let path: &str = "/v1/account";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match common::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            let header = Header::new("Authorization", format!("Bearer {}", tokens.access_token));
            let resp = client.get(format!("{}", path)).header(header).dispatch();

            assert_eq!(resp.status(), Status::Ok);

            let value = response_json_value(resp).to_string();
            let user_info: UserResp =
                serde_json::from_str(&value.as_str()).expect("account valid response");

            assert_eq!("upadawan", user_info.username);
            assert_eq!(Tariff::Free, user_info.tariff);
        }

        Err(err) => assert!(false, "\n\nFaild to login: {}\n\n", err),
    }
}

#[test]
fn refresh_token() {
    let path: &str = "/v1/account/token/refresh";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match common::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            // Обновляю токен
            {
                let resp = client
                    .post(format!("{}", path))
                    .body(json_string!({
                        "refresh_token": tokens.refresh_token
                    }))
                    .dispatch();

                assert_eq!(resp.status(), Status::Ok);

                let value = response_json_value(resp).to_string();
                let body: Tokens =
                    serde_json::from_str(&value.as_str()).expect("tokens valid response");

                let access_payload =
                    Tokens::decode_token::<AccessClaims>(body.access_token.as_str())
                        .expect("valid access token");

                let refresh_payload =
                    Tokens::decode_token::<RefreshClaims>(body.refresh_token.as_str())
                        .expect("valid access token");

                assert_eq!("upadawan", access_payload.claims.get_username());
                assert_eq!("upadawan", refresh_payload.claims.get_username());
            }

            // Пытаюсь еще раз обновить старый токен
            {
                let resp = client
                    .post(format!("{}", path))
                    .body(json_string!({
                        "refresh_token": tokens.refresh_token
                    }))
                    .dispatch();

                assert_eq!(resp.status(), Status::Unauthorized);
            }
        }

        Err(err) => assert!(false, "\n\nFaild to login: {}\n\n", err),
    }
}

#[test]
fn logout() {
    let path: &str = "/v1/account/logout";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match common::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            fn login(client: &Client, tokens: &Tokens, path: &str) -> Status {
                let header =
                    Header::new("Authorization", format!("Bearer {}", tokens.access_token));

                let resp = client
                    .post(format!("{}", path))
                    .header(header)
                    .body(json_string!({
                        "refresh_token": tokens.refresh_token
                    }))
                    .dispatch();

                resp.status()
            }

            // Выхожу из аккаунта
            assert_eq!(login(&client, &tokens, path), Status::Ok);

            // Пытаюсь снова войти со старыми токенами
            assert_eq!(login(&client, &tokens, path), Status::Unauthorized);
        }

        Err(err) => assert!(false, "\n\nFaild to login: {}\n\n", err),
    }
}

#[test]
fn delete_account() {
    let path: &str = "/v1/account/delete";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::new("delpad", "somepassword");

    match common::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            let header = Header::new("Authorization", format!("Bearer {}", tokens.access_token));
            let resp = client.delete(format!("{}", path)).header(header).dispatch();

            assert_eq!(resp.status(), Status::Ok);
        }

        Err(err) => assert!(false, "\n\nFaild to login: {}\n\n", err),
    }
}

mod auth_guard {
    use crate::{common, common::accounts::TestPadawan};
    use rocket::http::{Header, Status};

    #[test]
    fn invalid_format_without_bearer() {
        let path: &str = "/v1/account";
        let client = common::test_client().lock().unwrap();
        let padawan = TestPadawan::default();

        match common::try_login(&client, Box::new(padawan)) {
            Ok(tokens) => {
                let header = Header::new("Authorization", format!("{}", tokens.access_token));

                let resp = client.get(format!("{}", path)).header(header).dispatch();

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
        let padawan = TestPadawan::default();

        match common::try_login(&client, Box::new(padawan)) {
            Ok(_) => {
                let header = Header::new("Authorization", format!("Bearer"));

                let resp = client.get(format!("{}", path)).header(header).dispatch();

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
        let padawan = TestPadawan::default();

        match common::try_login(&client, Box::new(padawan)) {
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
        let padawan = TestPadawan::default();

        match common::try_login(&client, Box::new(padawan)) {
            Ok(tokens) => {
                let header =
                    Header::new("Authorization", format!("Bearer {}", tokens.access_token));

                let resp = client.get(format!("{}", path)).header(header).dispatch();

                assert_eq!(resp.status(), Status::Ok);
            }

            Err(err) => {
                assert!(false, "\n\nFaild to login: {}\n\n", err)
            }
        }
    }
}
