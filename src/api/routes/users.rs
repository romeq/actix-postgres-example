use actix_web::{get, Responder};

#[get("/")]
pub async fn get() -> impl Responder {
    "Hi!"
}
