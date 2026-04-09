#![allow(dead_code)]

use crate::{errors::ApiError, middleware::auth_middleware::extract_claims};
use actix_web::HttpRequest;

/// Guard: ensure authenticated user has a specific role.
/// Usage in handlers: `require_role(&req, "creator")?;`
pub fn require_role(req: &HttpRequest, role: &str) -> Result<(), ApiError> {
    let claims = extract_claims(req)?;
    if claims.has_role(role) || claims.is_admin() {
        Ok(())
    } else {
        Err(ApiError::Forbidden(
            t!("middleware.role_required", role = role).into_owned(),
        ))
    }
}

/// Guard: ensure the authenticated user IS the resource owner or an admin.
pub fn require_owner_or_admin(
    req: &HttpRequest,
    owner_profile_id: uuid::Uuid,
) -> Result<(), ApiError> {
    let claims = extract_claims(req)?;
    let requester = claims
        .profile_id()
        .ok_or(ApiError::Forbidden(t!("middleware.profile_id_not_found").into_owned()))?;
    if requester == owner_profile_id || claims.is_admin() {
        Ok(())
    } else {
        Err(ApiError::Forbidden(
            t!("middleware.permission_denied").into_owned(),
        ))
    }
}
