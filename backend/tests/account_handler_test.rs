mod common;

use jokehub::model::account::Theme;
use rocket::http::{ContentType, Header, Status};
use rocket::local::blocking::Client;
use serde::Deserialize;

use common::accounts::TestPadawan;
use jokehub::model::account::{
    security::{AccessClaims, RefreshClaims, Tokens},
    Account, Tariff,
};

use common::accounts as account;

#[test]
fn auth() {
    let client = common::test_client().lock().unwrap();

    // Регистрация пользователя
    {
        #[derive(Deserialize, Debug)]
        struct RegResp {
            #[allow(dead_code)]
            id: String,
        }

        let path: &str = "/v1/registration";
        let resp = client
            .post(path)
            .header(ContentType::JSON)
            .body(json_string!({
                "username": "I0HuKc",
                "password": "1234password"
            }))
            .dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_body!(resp, RegResp);
    }

    // Авторизация пользователя
    {
        let path: &str = "/v1/login";
        let resp = client
            .post(format!("{}", path))
            .header(ContentType::JSON)
            .body(json_string!({
                "username": "I0HuKc",
                "password": "1234password"
            }))
            .dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_body!(resp, Tokens);
    }
}

#[test]
fn account_padawan() {
    let path: &str = "/v1/account";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match account::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            let resp = client
                .get(format!("{}", path))
                .header(bearer!((tokens.access_token)))
                .dispatch();

            assert_eq!(resp.status(), Status::Ok);

            let body = assert_body!(resp, Account);

            assert_eq!("upadawan", body.get_username());
            assert_eq!(Tariff::Free, *body.get_tariff());
        }

        Err(err) => assert!(false, "\n\nFaild to login: {}\n\n", err),
    }
}

#[test]
fn refresh_token() {
    let path: &str = "/v1/account/token/refresh";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match account::try_login(&client, Box::new(padawan)) {
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

                let body = assert_body!(resp, Tokens);

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
fn api_key_regen() {
    let path: &str = "/v1/account/api-key/key";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match account::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            let old_api_key: String;

            // Получаю старый api ключ
            {
                let resp = client
                    .get("/v1/account")
                    .header(bearer!((tokens.access_token)))
                    .dispatch();

                assert_eq!(resp.status(), Status::Ok);

                let body = assert_body!(resp, Account);
                old_api_key = body.api_key;
            }

            // Обновляю ключ
            {
                let resp = client
                    .put(path)
                    .header(bearer!((tokens.access_token)))
                    .dispatch();

                assert_eq!(resp.status(), Status::Ok);

                #[derive(Deserialize)]
                struct Resp {
                    new_key: String,
                }

                let body = assert_body!(resp, Resp);
                assert_ne!(body.new_key, old_api_key);
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

    match account::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            fn login(client: &Client, tokens: &Tokens, path: &str) -> Status {
                let resp = client
                    .post(format!("{}", path))
                    .header(bearer!((tokens.access_token)))
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

    match account::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            let resp = client
                .delete(format!("{}", path))
                .header(bearer!((tokens.access_token)))
                .dispatch();

            assert_eq!(resp.status(), Status::Ok);
        }

        Err(err) => assert!(false, "\n\nFaild to login: {}\n\n", err),
    }
}

#[test]
fn change_password() {
    let path: &str = "/v1/account/password/change";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::new("changepass", "password1234");

    match account::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            // Пытаюсь сменить пароль на тот же
            {
                let resp = client
                    .post(format!("{}", path))
                    .body(json_string!({
                        "old_password": "password1234",
                        "new_password": "password1234"
                    }))
                    .header(bearer!((tokens.access_token)))
                    .dispatch();

                assert_eq!(resp.status(), Status::UnprocessableEntity);
            }

            {
                let resp = client
                    .post(format!("{}", path))
                    .body(json_string!({
                        "old_password": "password1234",
                        "new_password": "password4321"
                    }))
                    .header(bearer!((tokens.access_token)))
                    .dispatch();

                assert_eq!(resp.status(), Status::Ok);
            }

            // Проверяю что пароль действительно изменен
            {
                let resp = client
                    .post("/v1/login")
                    .body(json_string!({
                        "username": "changepass",
                        "password": "password1234" // использую старый пароль
                    }))
                    .header(ContentType::JSON)
                    .header(bearer!((tokens.access_token)))
                    .dispatch();

                assert_eq!(resp.status(), Status::NotFound);
            }

            // Пытаюсь войти с новым паролем
            let padawan = TestPadawan::new("changepass", "password4321");
            match account::try_login(&client, Box::new(padawan)) {
                Ok(tokens) => {
                    // Проверяю что до этого все сессии были удалены и сейчас существует
                    //только текущая и та которая была создана в момент смены пароля.
                    let resp = client
                        .get("/v1/account")
                        .header(bearer!((tokens.access_token)))
                        .dispatch();

                    assert_eq!(resp.status(), Status::Ok);

                    let body = assert_body!(resp, Account);

                    assert_eq!(body.sessions.len(), 2);
                }
                Err(err) => assert!(false, "\n\nFaild to login with new password: {}\n\n", err),
            }
        }

        Err(err) => assert!(false, "\n\nFaild to login: {}\n\n", err),
    }
}

#[test]
fn change_theme() {
    let path: &str = "/v1/account/theme/to/dark";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match account::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            let resp = client
                .put(path)
                .header(bearer!((tokens.access_token)))
                .dispatch();

            assert_eq!(resp.status(), Status::Ok);

            // Проверяю что тема действительно обновилась
            {
                let resp = client
                    .get("/v1/account")
                    .header(bearer!((tokens.access_token)))
                    .dispatch();

                assert_eq!(resp.status(), Status::Ok);

                let body = assert_body!(resp, Account);
                assert_eq!(*body.get_theme(), Theme::Dark);
            }
        }
        Err(err) => assert!(false, "\n\nFaild to login: {}\n\n", err),
    }
}

#[test]
fn logout_any() {
    let path: &str = "/v1/account/logout/any";
    let client = common::test_client().lock().unwrap();
    let padawan = TestPadawan::default();

    match account::try_login(&client, Box::new(padawan)) {
        Ok(tokens) => {
            let resp = client
                .post(path)
                .header(bearer!((tokens.access_token)))
                .dispatch();

            assert_eq!(resp.status(), Status::Ok);

            // Проверю что все сессии удалены
            {
                let resp = client
                    .get("/v1/account")
                    .header(bearer!((tokens.access_token)))
                    .dispatch();

                assert_eq!(resp.status(), Status::Ok);

                let body = assert_body!(resp, Account);

                assert_eq!(body.sessions.len(), 0);
            }
        }
        Err(err) => assert!(false, "\n\nFaild to login: {}\n\n", err),
    }
}

mod tariff_guard {
    use crate::{assert_body, bearer, common, common::joke::TestNewJoke, TestPadawan};
    use rocket::http::{Header, Status};
    use serde::Deserialize;

    #[test]
    fn get_record_by_tariff_free() {
        let path: &str = "/v1/joke";
        let client = common::test_client().lock().unwrap();
        let padawan = TestPadawan::default();

        match TestNewJoke::create_test_record(&client, Box::new(padawan)) {
            Ok((tokens, status, id)) => {
                assert_eq!(status, Status::Ok);

                let resp = client
                    .get(format!("{}/{}", path, id))
                    .header(bearer!((tokens.access_token)))
                    .dispatch();

                assert_eq!(resp.status(), Status::Ok);

                #[allow(dead_code)]
                #[derive(Deserialize, Debug)]
                struct Response {
                    category: String,
                    text: String,
                }

                assert_body!(resp, Response);
            }

            Err(err) => assert!(false, "\n\nFaild to create test record: {}\n\n", err),
        }
    }
}

mod level_guard {
    use crate::{
        common,
        common::accounts as account,
        common::accounts::{TestMaster, TestPadawan, TestSith},
    };
    use jokehub::model::account::security::{AccessClaims, Tokens};
    use rocket::http::{Header, Status};

    #[test]
    fn level_upgrade_by_padawan() {
        let path: &str = "/v1/privilege/tsith/padawan";
        let client = common::test_client().lock().unwrap();
        let padawan = TestPadawan::default();

        match account::try_login(&client, Box::new(padawan)) {
            Ok(tokens) => {
                let resp = client
                    .put(format!("{}", path))
                    .header(crate::bearer!((tokens.access_token)))
                    .dispatch();

                assert_eq!(resp.status(), Status::Forbidden);
            }

            Err(err) => {
                assert!(false, "\n\nFaild to login: {}\n\n", err)
            }
        }
    }

    #[test]
    fn level_upgrade_by_master() {
        let path: &str = "/v1/privilege/tsith/padawan";
        let client = common::test_client().lock().unwrap();
        let master = TestMaster::default();

        match account::try_login(&client, Box::new(master)) {
            Ok(tokens) => {
                let resp = client
                    .put(format!("{}", path))
                    .header(crate::bearer!((tokens.access_token)))
                    .dispatch();

                assert_eq!(resp.status(), Status::Forbidden);
            }

            Err(err) => {
                assert!(false, "\n\nFaild to login: {}\n\n", err)
            }
        }
    }

    #[test]
    fn level_upgrade_by_sith() {
        let path: &str = "/v1/privilege/tpadawan/master";
        let client = common::test_client().lock().unwrap();
        let sith = TestSith::default();

        match account::try_login(&client, Box::new(sith)) {
            Ok(tokens) => {
                let resp = client
                    .put(format!("{}", path))
                    .header(crate::bearer!((tokens.access_token)))
                    .dispatch();

                assert_eq!(resp.status(), Status::Ok);

                // Проверка действительности обновления уровня
                {
                    let updated_user = TestPadawan::new("tpadawan", "12344321e");
                    match account::try_login(&client, Box::new(updated_user)) {
                        Ok(tokens) => {
                            let access_payload =
                                Tokens::decode_token::<AccessClaims>(tokens.access_token.as_str())
                                    .expect("valid access token");

                            assert_eq!(
                                access_payload.claims.get_level().to_string().to_lowercase(),
                                "master".to_string()
                            );
                        }
                        Err(err) => {
                            assert!(false, "\n\nFaild to login by updated user: {}\n\n", err)
                        }
                    }
                }
            }

            Err(err) => {
                assert!(false, "\n\nFaild to login: {}\n\n", err)
            }
        }
    }
}

mod auth_guard {
    use crate::{common, common::accounts as account, common::accounts::TestPadawan};
    use rocket::http::{Header, Status};

    #[test]
    fn invalid_format_without_bearer() {
        let path: &str = "/v1/account";
        let client = common::test_client().lock().unwrap();
        let padawan = TestPadawan::default();

        match account::try_login(&client, Box::new(padawan)) {
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

        match account::try_login(&client, Box::new(padawan)) {
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

        match account::try_login(&client, Box::new(padawan)) {
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

        match account::try_login(&client, Box::new(padawan)) {
            Ok(tokens) => {
                let resp = client
                    .get(format!("{}", path))
                    .header(crate::bearer!((tokens.access_token)))
                    .dispatch();

                assert_eq!(resp.status(), Status::Ok);
            }

            Err(err) => {
                assert!(false, "\n\nFaild to login: {}\n\n", err)
            }
        }
    }
}
