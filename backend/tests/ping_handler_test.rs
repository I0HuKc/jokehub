use rocket::http::Status;

mod common;

#[test]
fn ping() {
    let client = common::test_client().lock().unwrap();
    let resp = client.get("/ping").dispatch();

    assert_eq!(resp.status(), Status::Ok);
}
