use dotenv::dotenv;

use jokehub::server::rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let _server = rocket().launch().await?;

    Ok(())
}
