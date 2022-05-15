use rocket::http::Status;

use super::common::*;

#[test]
fn ping() {
    let client = test_client().lock().unwrap();
    let resp = client.get("/ping").dispatch();

    let s = resp.status();
    assert_eq!(s, Status::Ok);
}
