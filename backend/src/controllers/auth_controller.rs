use crate::{
    config::AppConfig,
    db::DbPool,
    errors::{AppError, AppResult},
    middleware::create_token,
    models::{NewUser, User},
    schema::users,
};
use actix_web::{web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── DTOs ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email:    String,
    pub password: String,
    pub role:     Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token:    String,
    pub user_id:  Uuid,
    pub username: String,
    pub role:     String,
}

// ── Handlers ─────────────────────────────────────────────────

/// POST /api/auth/login
pub async fn login(
    pool:   web::Data<DbPool>,
    config: web::Data<AppConfig>,
    body:   web::Json<LoginRequest>,
) -> AppResult<HttpResponse> {
    let mut conn = pool.get().map_err(|e| AppError::Internal(e.to_string()))?;
    let username  = body.username.clone();
    let password  = body.password.clone();

    let user: User = web::block(move || {
        users::table
            .filter(users::username.eq(&username))
            .filter(users::active.eq(true))
            .select(User::as_select())
            .first(&mut conn)
            .optional()
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .map_err(AppError::Database)?
    .ok_or(AppError::Unauthorized)?;

    let valid = verify(&password, &user.password)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if !valid {
        return Err(AppError::Unauthorized);
    }

    let token = create_token(
        user.id,
        user.username.clone(),
        user.role.clone(),
        &config.jwt_secret,
        config.jwt_expiry_hours,
    )?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user_id:  user.id,
        username: user.username,
        role:     user.role,
    }))
}

/// POST /api/auth/register  (admin only in production — open for setup)
pub async fn register(
    pool: web::Data<DbPool>,
    body: web::Json<RegisterRequest>,
) -> AppResult<HttpResponse> {
    let mut conn = pool.get().map_err(|e| AppError::Internal(e.to_string()))?;

    let password_hash = hash(&body.password, DEFAULT_COST)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let now  = Utc::now();
    let role = body.role.clone().unwrap_or_else(|| "operator".to_string());

    let new_user = NewUser {
        id:         Uuid::new_v4(),
        username:   body.username.clone(),
        email:      body.email.clone(),
        password:   password_hash,
        role,
        active:     true,
        created_at: now,
        updated_at: now,
    };

    let user: User = web::block(move || {
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .map_err(|e| match e {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation, _
        ) => AppError::Conflict("Usuário ou e-mail já existe".to_string()),
        other => AppError::Database(other),
    })?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "id":       user.id,
        "username": user.username,
        "email":    user.email,
        "role":     user.role,
    })))
}

/// GET /api/auth/me
pub async fn me(user: crate::middleware::AuthUser) -> AppResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id":       user.claims().sub,
        "username": user.claims().username,
        "role":     user.claims().role,
    })))
}
