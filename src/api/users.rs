use crate::{
    database::{db::Database, users::*},
    errors::*,
};
use actix_web::{
    post,
    web::{block, Data, Json},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct NewUserResponse {
    message: String,
}

#[post("/create")]
pub async fn new_account(
    user: Json<NewUserRequest>,
    db: Data<Database>,
) -> Result<Json<NewUserResponse>, UserError> {
    block(move || db.get()?.create_user(user.0)).await??;

    Ok(Json(NewUserResponse {
        message: String::from("created!"),
    }))
}

#[derive(Serialize)]
pub struct LoginResponse {
    message: String,
}

#[post("/login")]
pub async fn login(
    user: Json<LoginRequest>,
    db: Data<Database>,
) -> Result<Json<LoginResponse>, UserError> {
    block(move || db.get()?.login(user.0)).await??;

    Ok(Json(LoginResponse {
        message: String::from("logged in"),
    }))
}
