pub mod db;
pub mod errors;
pub mod model;
pub mod server;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate dotenv_codegen;

extern crate zxcvbn;

pub trait VectorConvert<F> {
    fn convert<T>(from: Vec<F>) -> Vec<T>
    where
        T: From<F>;
}

impl<F> VectorConvert<F> for Vec<F> {
    fn convert<T>(from: Vec<F>) -> Vec<T>
    where
        T: From<F>,
    {
        let mut result: Vec<T> = Vec::new();
        from.into_iter().for_each(|item| result.push(item.into()));
        result
    }
}
