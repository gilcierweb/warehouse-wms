use actix_web::{HttpResponse, error::ResponseError};
use diesel::result::Error as DieselError;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Internal(String),
    Database(String),
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Internal(msg) => write!(f, "Internal Error: {}", msg),
            AppError::Database(msg) => write!(f, "Database Error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Internal(msg) => HttpResponse::InternalServerError().body(msg.clone()),
            AppError::Database(msg) => HttpResponse::InternalServerError().body(msg.clone()),
            AppError::NotFound(msg) => HttpResponse::NotFound().body(msg.clone()),
            AppError::Conflict(msg) => HttpResponse::Conflict().body(msg.clone()),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().body(msg.clone()),
            AppError::BadRequest(msg) => HttpResponse::BadRequest().body(msg.clone()),
        }
    }
}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        AppError::Database(err.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
