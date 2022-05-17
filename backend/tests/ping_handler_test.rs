use rocket::http::Status;

mod common;

#[test]
fn ping() {
    let client = common::test_client().lock().unwrap();
    let resp = client.get("/ping").dispatch();

    let s = resp.status();
    assert_eq!(s, Status::Ok);
}
