use actix_identity::Identity;
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use crate::{database::db::Database, database::files::File, errors::*};
use actix_web::{
    dev::Payload,
    post,
    web::{block, Data, Json},
    FromRequest, HttpRequest,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggedUser {}

impl FromRequest for LoggedUser {
    type Error = UserError;
    type Future = Ready<Result<LoggedUser, Self::Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Ok(user_json) = identity.id() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ready(Ok(user));
                }
            }
        }

        ready(Err(UserError::PermissionDenied.into()))
    }
}

#[post("/create/{filename}")]
pub async fn upload_file(
    filename: actix_web::web::Path<String>,
    user: LoggedUser,
    db: Data<Database>,
) -> Result<Json<()>, UserError> {
    let file: File = File {
        original_filename: filename.to_lowercase(),
        owner_id: uuid::Uuid::new_v4(),
        uploaded_at: Some(std::time::SystemTime::now()),
        was_encrypted: false,
    };
    println!("{:?}", user);

    block(move || db.get()?.create_file(file)).await??;

    Ok(Json(()))
}
