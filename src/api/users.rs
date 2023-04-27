use crate::{
    database::db::Database,
    errors::*,
    models::{NewUser, User, UserLogin},
};
use actix_identity::Identity;
use actix_web::{
    get, post,
    web::{block, Data, Json},
    HttpMessage, HttpRequest,
};

#[post("/register")]
pub async fn new_account(
    request: HttpRequest,
    user: Json<NewUser>,
    db: Data<Database>,
) -> Result<String, UserError> {
    let user = block(move || db.get()?.register(user.0)).await??;
    Identity::login(&request.extensions(), user.as_simple().to_string())
        .map_err(|_| UserError::InternalError)?;
    Ok(String::from(""))
}

#[post("/login")]
pub async fn login(
    request: HttpRequest,
    user: Json<UserLogin>,
    db: Data<Database>,
) -> Result<String, UserError> {
    let id = block(move || db.get()?.login(user.0)).await??;
    Identity::login(&request.extensions(), id.as_simple().to_string())
        .map_err(|_| UserError::InternalError)?;

    Ok(String::from(""))
}

#[get("/profile")]
pub async fn show_user(user: Identity, db: Data<Database>) -> Result<Json<User>, UserError> {
    let user = user.id().map_err(|_| UserError::PermissionDenied)?;
    let id = uuid::Uuid::try_parse(user.as_str()).map_err(|_| UserError::PermissionDenied)?;
    let profile = block(move || db.get()?.profile(id)).await??;

    Ok(Json(profile))
}
