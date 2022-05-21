use rocket::config::Config;
use rocket::figment::Figment;

// pub struct Lingua<'a>(pub &'a State<Box<LanguageDetector>>);

// pub fn manage_lingua(self) -> Rocket<Build> {
//     let languages = vec![English, Russian];
//     let detector: Box<LanguageDetector> =
//         Box::new(LanguageDetectorBuilder::from_languages(&languages).build());
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for Lingua<'r> {
//     type Error = ();

//     async fn from_request(request: &'r Request<'_>) -> request::Outcome<Lingua<'r>, Self::Error> {
//         let outcome = request.guard::<&State<Box<LanguageDetector>>>().await;
//         match outcome {
//             Outcome::Success(client) => Outcome::Success(Lingua(client)),
//             Outcome::Failure(_) => todo!(),
//             Outcome::Forward(_) => todo!(),
//         }
//     }
// }

pub fn from_env() -> Figment {
    let port = dotenv!("SERVER_PORT").parse::<u16>().unwrap();

    Config::figment()
        .merge(("address", dotenv!("SERVER_HOST")))
        .merge(("port", port))
}
