use actix_web::{HttpResponse, delete, get, patch, web};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::profile::{NewProfile, Profile},
    models::refresh_token::RefreshToken,
    repositories::container::AppContainer,
};

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub slug: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserMeResponse {
    pub user: UserInfo,
    pub profile: Profile,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
}

/// GET /api/users/me
#[get("/users/me")]
pub async fn get_me(user: AuthUser, container: web::Data<AppContainer>) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;
    let user_data = container
        .users
        .find(&user_id)
        .await
        .map_err(|_| AppError::NotFound(t!("users.not_found").into_owned()))?;

    let profile = container
        .profiles
        .find_by_user_id(&user_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(t!("users.profile_not_found").into_owned()))?;

    let roles = container
        .users
        .get_user_roles(&user_id)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(UserMeResponse {
        user: UserInfo {
            id: user_data.id,
            email: user_data.email,
        },
        profile,
        roles,
    }))
}

/// PATCH /api/users/me
#[patch("/users/me")]
pub async fn update_me(
    user: AuthUser,
    container: web::Data<AppContainer>,
    body: web::Json<UpdateProfileRequest>,
) -> AppResult<HttpResponse> {
    let profile_id = user.claims().profile_id;

    let profile = container
        .profiles
        .find(&profile_id)
        .await
        .map_err(|_| AppError::NotFound(t!("users.profile_not_found").into_owned()))?;

    // Simplified profile update using only fields available in the current Profile model
    let updated_profile = NewProfile {
        user_id: profile.user_id,
        first_name_enc: profile.first_name_enc.clone(),
        last_name_enc: profile.last_name_enc.clone(),
        phone_enc: profile.phone_enc.clone(),
        full_name: profile.full_name.clone(),
        nickname: body.display_name.clone().or(profile.nickname.clone()),
        bio: body.bio.clone().or(profile.bio.clone()),
        birthday: profile.birthday,
        avatar: body.avatar_url.clone().or(profile.avatar.clone()),
        phone: profile.phone,
        social_network: profile.social_network.clone(),
        status: profile.status,
    };

    let updated = container
        .profiles
        .update(&profile_id, &updated_profile)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(updated))
}

/// GET /api/users/me/sessions
#[get("/users/me/sessions")]
pub async fn list_sessions(
    user: AuthUser,
    container: web::Data<AppContainer>,
    pagination: web::Query<crate::utils::PaginationParams>,
) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;

    let all_tokens = container
        .refresh_tokens
        .all()
        .await
        .map_err(AppError::Database)?;

    let sessions: Vec<&RefreshToken> = all_tokens
        .iter()
        .filter(|t| t.user_id == user_id && t.revoked_at.is_none() && t.expires_at > Utc::now())
        .collect();

    let total = sessions.len() as i64;
    let response =
        crate::utils::PaginatedResponse::new(sessions, total, pagination.page, pagination.per_page);

    Ok(HttpResponse::Ok().json(response))
}

/// DELETE /api/users/me/sessions/{id}
#[delete("/users/me/sessions/{id}")]
pub async fn revoke_session(
    user: AuthUser,
    container: web::Data<AppContainer>,
    path: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;
    let session_id = path.into_inner();

    // Verify the session belongs to the user
    let token = container
        .refresh_tokens
        .find(&session_id)
        .await
        .map_err(|_| AppError::NotFound(t!("users.session_not_found").into_owned()))?;

    if token.user_id != user_id {
        return Err(AppError::Forbidden(t!("users.not_authorized").into_owned()));
    }

    container
        .refresh_tokens
        .revoke(&session_id)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "revoked": true })))
}

/// DELETE /api/users/me
#[delete("/users/me")]
pub async fn delete_me(user: AuthUser, container: web::Data<AppContainer>) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;

    container
        .refresh_tokens
        .revoke_all_for_user(&user_id)
        .await
        .map_err(AppError::Database)?;

    match container.users.destroy(&user_id).await {
        Ok(0) => Err(AppError::NotFound(t!("users.not_found").into_owned())),
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "deleted": true,
            "message": t!("users.account_deleted")
        }))),
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::ForeignKeyViolation,
            _,
        )) => Err(AppError::Conflict(
            t!("users.account_delete_conflict").into_owned(),
        )),
        Err(e) => Err(AppError::Database(e)),
    }
}
