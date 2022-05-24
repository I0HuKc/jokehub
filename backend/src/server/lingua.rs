use lingua::{
    Language,
    Language::{English, Russian},
    LanguageDetector, LanguageDetectorBuilder,
};
use rocket::{
    outcome::Outcome,
    request::{self, FromRequest},
    Build, Request, Rocket, State,
};

use crate::errors::HubError;

pub struct Lingua<'a>(pub &'a State<Box<LanguageDetector>>);

impl<'a> Lingua<'a> {
    pub fn detected<T>(self, text: T) -> Result<Language, HubError>
    where
        T: Into<String>,
    {
        match self.0.detect_language_of(text) {
            Some(language) => match language {
                English => Ok(Language::English),
                Russian => Ok(Language::Russian),
            },

            None => {
                let error = HubError::new_unprocessable(
                    "Unable to determine what language is used in the text",
                    None,
                );

                Err(error)
            }
        }
    }
}

pub trait LinguaManage {
    fn manage_lingua(self) -> Self;
}

impl LinguaManage for Rocket<Build> {
    fn manage_lingua(self) -> Self {
        let languages = vec![English, Russian];
        let detector: Box<LanguageDetector> =
            Box::new(LanguageDetectorBuilder::from_languages(&languages).build());

        self.manage(detector)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Lingua<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Lingua<'r>, Self::Error> {
        let outcome = request.guard::<&State<Box<LanguageDetector>>>().await;
        match outcome {
            Outcome::Success(client) => Outcome::Success(Lingua(client)),
            Outcome::Failure(_) => todo!(),
            Outcome::Forward(_) => todo!(),
        }
    }
}
