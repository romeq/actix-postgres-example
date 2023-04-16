use actix_web::{
    error::BlockingError,
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Internal error")]
    InternalError,
    #[error("Request includes invalid body")]
    InvalidBody,
}

impl std::convert::From<BlockingError> for UserError {
    fn from(_: BlockingError) -> Self {
        UserError::InternalError
    }
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}
