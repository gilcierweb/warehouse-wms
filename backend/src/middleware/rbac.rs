use uuid::Uuid;

use crate::errors::{AppError, AppResult};

use super::auth::Claims;

pub fn require_admin(claims: &Claims) -> AppResult<()> {
    if claims.is_admin() {
        Ok(())
    } else {
        Err(AppError::Forbidden(t!("users.not_authorized").into_owned()))
    }
}

pub fn require_owner_or_admin(claims: &Claims, owner_id: Uuid) -> AppResult<()> {
    if claims.sub == owner_id || claims.is_admin() {
        Ok(())
    } else {
        Err(AppError::Forbidden(t!("users.not_authorized").into_owned()))
    }
}
