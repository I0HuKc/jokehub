use mongodb::{bson::doc, sync::Client};
use rocket::serde::json::Json;
use rocket::http::{CookieJar, Cookie};
use rocket::State;
use serde_json::{json, Value};
use validator::Validate;

use crate::{
    db::mongo::{varys::Varys, Crud},
    errors::{ErrorChapter, Errors, ErrorsKind, CH_DATABASE},
    model::{
        account::{security::Tokens, *},
        uuid_validation,
    },
};


#[post("/registration", data = "<jnu>")]
pub async fn registration<'f>(client: &State<Box<Client>>, jnu: Json<NewUser>) -> Result<Value, Errors<'f>> {
    jnu.0.validate()?;

    let result = User::create(
        Varys::get(client, Varys::Users),
        User::from(jnu.0).password_hashing()?
    )?;

    let resp = json!({"id": result.inserted_id});
    Ok(resp)
}

#[post("/login", data = "<jnu>")]
pub async fn login<'f>(client: &State<Box<Client>>, jnu: Json<NewUser>, cookies: &CookieJar<'_>) -> Result<(), Errors<'f>> {
    jnu.0.validate()?;

    let result = User::get_by_username(
        Varys::get(client, Varys::Users),
        jnu.0.username,
    )?;

    match result.password_verify(format!("{}", jnu.0.password).as_bytes()) {
        Ok(v) => {
            if v {  
                let tokens = Tokens::new(result.username, result.role)?; 
                
                cookies.add(Cookie::new("at", tokens.access_token.clone()));  
                cookies.add(Cookie::new("rt", tokens.refresh_token.clone()));    

                Ok(())
            } else {           
                Err(Errors::new(ErrorsKind::NotFound(ErrorChapter(CH_DATABASE.clone()))))
            }

        },
        
        Err(err) => Err(err),
    }
}


#[get("/user/<id>")]
pub async fn get_user<'f>(client: &State<Box<Client>>, id: &str) -> Result<Json<User>, Errors<'f>> {
    uuid_validation(id)?;
    
    let result = User::get_by_id(
        Varys::get(client, Varys::Users),
        id,
    )?;

    Ok(Json(result))
}
