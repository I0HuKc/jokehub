use rocket::http::Status;

#[test]
fn ping() {
    let client = super::test_client().lock().unwrap();
    let resp = client.get("/ping").dispatch();

    let s = resp.status();
    assert_eq!(s, Status::Ok); 
}