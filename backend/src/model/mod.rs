pub mod joke;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SUPPORTED_LANGUAGES: Vec<&'static str> = ["ru", "en"].to_vec();
}
