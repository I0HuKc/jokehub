use std::ops::{Deref, DerefMut};

use r2d2;
use r2d2::PooledConnection;
use r2d2_redis::RedisConnectionManager;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{outcome::Outcome, Request, State};

type Pool = r2d2::Pool<RedisConnectionManager>;
type PooledConn = PooledConnection<RedisConnectionManager>;

pub struct RedisConn(pub PooledConn);

impl Deref for RedisConn {
    type Target = PooledConn;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RedisConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn init_pool() -> Pool {
    let manager = RedisConnectionManager::new(dotenv!("REDIS_DB_URL")).expect("connection manager");
    //let manager = RedisConnectionManager::new(format!("redis://user:{}@{}:{}/{}", redis_password redis_address, redis_port, redis_db)).expect("connection manager");

    match r2d2::Pool::builder().max_size(15).build(manager) {
        Ok(pool) => pool,
        Err(e) => panic!("Error: failed to create redis database pool {}", e),
    }
}
