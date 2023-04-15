use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
struct CustomError {
    message: &'static str,
}

impl actix_web::error::ResponseError for CustomError {}
