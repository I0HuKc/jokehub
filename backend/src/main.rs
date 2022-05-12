use dotenv::dotenv;

use jokehub::server::Server;

#[rocket::launch]
fn rocket() -> _ {
    dotenv().ok();

    Server::launch(rocket::build())
}
