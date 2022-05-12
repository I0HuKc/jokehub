pub mod anecdote;
pub mod joke;
pub mod shrimp;
pub mod punch;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SUPPORTED_LANGUAGES: Vec<&'static str> = ["ru", "en"].to_vec();
}
