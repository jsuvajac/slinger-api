use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "DuplicateValue: {}", _0)]
    DuplicateValue(String),

    #[display(fmt = "DuplicateValue: {}", _0)]
    NoDataFound(String),

    #[display(fmt = "BadId")]
    BadId,

    #[display(fmt = "GenericError: {}", _0)]
    GenericError(String),

    #[display(fmt = "AuthenticationError: {}", _0)]
    AuthenticationError(String),
}
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::BadId => HttpResponse::BadRequest().json("Invalid ID"),
            AppError::AuthenticationError(ref message) => {
                HttpResponse::Unauthorized().json(message)
            }
            AppError::DuplicateValue(ref message) => HttpResponse::BadRequest().json(message),
            AppError::GenericError(ref message) => HttpResponse::BadRequest().json(message),
            AppError::NoDataFound(ref message) => HttpResponse::NoContent().json(message),
        }
    }
}
