mod server;
mod model;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    server::launcher()
}
