use jokehub::server::Server;

#[rocket::launch]
fn rocket() -> _ {
    Server::launch(rocket::build())
}
