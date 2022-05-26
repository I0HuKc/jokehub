mod common;

use crate::common::response_json_value;
use rocket::http::Status;
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
