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
    #[error("User already exists or the request is invalid")]
    UserAlreadyExistsOrRequestInvalid,
    #[error("Invalid request")]
    InvalidRequest,
    #[error("Permission denied")]
    PermissionDenied,
}

impl std::convert::From<BlockingError> for UserError {
    fn from(_: BlockingError) -> Self {
        UserError::InternalError
    }
}

#[derive(serde::Serialize)]
struct JsonErrorResponse {
    error_message: String,
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(JsonErrorResponse {
                error_message: self.to_string(),
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UserAlreadyExistsOrRequestInvalid => StatusCode::CONFLICT,
            Self::PermissionDenied => StatusCode::FORBIDDEN,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}
