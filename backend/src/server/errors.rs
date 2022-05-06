use rocket::http::Status;

use crate::Error;

#[derive(Copy, Clone)]
struct ServerError<'a> {
    error: &'a Error,
    status: Status,
}
