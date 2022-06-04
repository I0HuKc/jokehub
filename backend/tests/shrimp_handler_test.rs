mod common;

use crate::common::response_json_value;
use jokehub::model::shrimp::{ReactionKind, Shrimp};
use rocket::http::{Header, Status};
use test_case::test_case;

#[test_case("", Status::Ok ; "empty_filter" )]
#[test_case("category=joke&tag=base", Status::NotFound ; "non_existent_tag" )]
#[test_case("category=joke&tag=for_test", Status::Ok ; "existent_tag" )]
#[test_case("category=joke&tag=for_test&author=shavedkiwi", Status::Ok ; "existent_tag__existent_author" )]
#[test_case("category=joke&tag=for_test&author=noex", Status::NotFound ; "existent_tag__not_existent_author" )]
#[test_case("category=joke&tag=for_test&author=shavedkiwi&lang=russian", Status::Ok ; "existent_tag__existent_author__valid_lang" )]
#[test_case("category=joke&tag=for_test&author=shavedkiwi&lang=english", Status::Ok ; "existent_tag__existent_author__valid_lang_2" )]
#[test_case("category=joke&author=shavedkiwi&lang=english", Status::Ok ; "existent_tag__existent_author__valid_lang_3" )]
#[test_case("category=joke&author=shavedkiwi&lang=fr", Status::NotFound ; "existent_tag__existent_author__invalid_lang" )]
#[test_case("category=joke&flag=sexist", Status::NotFound ; "non_existent_flag" )]
#[test_case("category=joke&flag=nsfw", Status::Ok ; "existent_flag" )]
#[test_case("category=joke&flag=religious&tag=for_test", Status::Ok ; "existent_flag_and_existent_tag" )]
#[test_case("category=joke&flag=religious&tag=n_for_test", Status::NotFound ; "existent_flag_and_not__existent_tag" )]
#[test_case("category=joke&flag=religious&author=shavedkiwi", Status::Ok ; "existent_flag_and_author" )]
#[test_case("category=joke&flag=racist&author=shavedkiwi&lang=english", Status::Ok ; "existent_flag_and_author__lang" )]
#[test_case("category=joke&flag=religious", Status::Ok ; "valid" )]
#[test_case("category=joke&flag=religious&flag=racist&lang=russian", Status::Ok ; "valid_1" )]
#[test_case("category=joke&flag=religious&flag=racist&lang=russian&tag=for_test", Status::Ok ; "valid_2" )]
fn get_random(filter: &str, status: Status) {
    let path: &str = "/v1/random";
    let client = common::test_client().lock().unwrap();

    let resp = client.get(format!("{}?{}", path, filter)).dispatch();

    let s = resp.status();

    let value = response_json_value(resp);
    println!("\n\n=============={:?}==============\n\n", value);

    assert_eq!(s, status);
}

/// Тест предполагает использование токена с тарифом не ниже STANDART
#[test_case(ReactionKind::Laughing ; "reaction_laughing" )]
#[test_case(ReactionKind::Enraged ; "reaction_enraged" )]
#[test_case(ReactionKind::Fire ; "reaction_fire" )]
#[test_case(ReactionKind::ThumbsUp ; "reaction_thumbs_up" )]
#[test_case(ReactionKind::ThumbsDown ; "reaction_thumbs_down" )]
fn shrimp_reaction(reaction: ReactionKind) {
    let path = format!(
        "/v1/joke/reaction/11b923b0-4241-4c32-ac06-f560468fac20/{}",
        reaction.to_string().to_lowercase()
    );

    let client = common::test_client().lock().unwrap();

    let resp = client
        .post(path)
        .header(apikey!(
            "5Jh0Y7u6zJfK1PDdbd1GiJ9ahvoHoJz55FfmQQr8oSz7dcoi3o"
        ))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    {
        // Проверка действительности инкрементирования поля

        let resp = client
            .get("/v1/joke/11b923b0-4241-4c32-ac06-f560468fac20")
            .header(apikey!(
                "5Jh0Y7u6zJfK1PDdbd1GiJ9ahvoHoJz55FfmQQr8oSz7dcoi3o"
            ))
            .dispatch();

        assert_eq!(resp.status(), Status::Ok);

        #[allow(unused_parens)]
        let body = assert_body!(resp, (Shrimp<jokehub::model::joke::Joke>));
        assert_eq!(body.tail.reactions[&reaction], 1)
    }
}
