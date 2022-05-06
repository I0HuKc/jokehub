use diesel::RunQueryDsl;

use crate::{
    db::Conn,
    model::joke::{Joke, NewJoke},
    schema::jokes_tb,
    Error,
};

pub enum NewJokeOutcome {
    Ok(Joke),
    Other(Error),
}

pub async fn create(conn: Conn, nj: NewJoke) -> NewJokeOutcome {
    conn.run(move |c| {
        match diesel::insert_into(jokes_tb::table)
            .values(nj)
            .get_result::<Joke>(c)
        {
            Ok(j) => NewJokeOutcome::Ok(j),
            Err(e) => NewJokeOutcome::Other(Error::from(e)),
        }
    })
    .await
}