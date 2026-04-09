use derive_more::Display;
use diesel::result::Error as DieselError;
use std::error::Error;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "{_0}")]
    NotFound(String),

    #[display(fmt = "{_0}")]
    BadRequest(String),

    #[display(fmt = "{_0}")]
    Unauthorized(String),

    #[display(fmt = "{_0}")]
    Forbidden(String),

    #[display(fmt = "{_0}")]
    Conflict(String),

    #[display(fmt = "{_0}")]
    Validation(String),

    #[display(fmt = "{_0}")]
    Database(DieselError),

    #[display(fmt = "{_0}")]
    Internal(String),
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Database(e) => Some(e),
            _ => None,
        }
    }
}
