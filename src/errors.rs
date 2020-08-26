use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use uuid::Error as UuidError;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "DuplicateValue: {}", _0)]
    DuplicateValue(String),

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
        }
    }
}
impl From<UuidError> for AppError {
    fn from(_: UuidError) -> AppError {
        AppError::BadId
    }
}
impl From<DBError> for AppError {
    fn from(error: DBError) -> AppError {
        // We only care about UniqueViolations
        match error {
            DBError::DatabaseError(kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();
                match kind {
                    DatabaseErrorKind::UniqueViolation => AppError::DuplicateValue(message),
                    _ => AppError::GenericError(message),
                }
            }
            _ => AppError::GenericError(String::from("Some database error occured")),
        }
    }
}
