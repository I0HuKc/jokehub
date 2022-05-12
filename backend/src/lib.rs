pub mod db;
pub mod model;
pub mod schema;
pub mod server;
pub mod errors;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;
