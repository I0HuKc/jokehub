use diesel::RunQueryDsl;

use crate::{
    db::Conn,
    model::joke::{Joke, NewJoke},
    schema::jokes_tb,
    Errors, 
};

impl Joke {
    pub async fn create(conn: Conn, nj: NewJoke) -> Result<Joke, Errors> {
        let joke = conn
            .run(move |c| {
                diesel::insert_into(jokes_tb::table)
                    .values(nj)
                    .get_result::<Joke>(c)
            })
            .await?;

        Ok(joke)
    }
}
