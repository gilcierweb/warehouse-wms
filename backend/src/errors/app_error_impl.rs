use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde_json::json;

use crate::errors::AppError;

impl From<DieselError> for AppError {
    fn from(error: DieselError) -> Self {
        AppError::Database(error)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => actix_web::http::StatusCode::FORBIDDEN,
            AppError::Conflict(_) => actix_web::http::StatusCode::CONFLICT,
            AppError::Validation(_) => actix_web::http::StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Database(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_code = match self {
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::Forbidden(_) => "FORBIDDEN",
            AppError::Conflict(_) => "CONFLICT",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::Database(_) => "DB_ERROR",
            AppError::Internal(_) => "INTERNAL_ERROR",
        };

        HttpResponse::build(self.status_code()).json(json!({
            "error": {
                "code":    error_code,
                "message": self.to_string(),
            }
        }))
    }
}
