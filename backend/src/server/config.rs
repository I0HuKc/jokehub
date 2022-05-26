use rocket::config::Config;
use rocket::figment::Figment;

pub fn from_env() -> Figment {
    let port = dotenv!("SERVER_PORT").parse::<u16>().unwrap();

    Config::figment()
        .merge(("address", dotenv!("SERVER_HOST")))
        .merge(("port", port))
}
