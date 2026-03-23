use crate::{
    auth::password::{password_hash, verify},
    config::AppConfig,
    db::database::DBPool,
    errors::{AppError, AppResult},
    middleware::auth::{create_token, AuthUser, ROLE_OPERATOR},
    models::user::{NewUser, User},
    db::schema::users,
};
use actix_web::{web, HttpResponse, post, get};
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
    pub role:     Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token:    String,
    pub user_id:  Uuid,
    pub username: String,
    pub role:     i32,
}

// ── Handlers ─────────────────────────────────────────────────

// POST /api/auth/login
#[post("/login")]
pub async fn login(
    pool:   web::Data<DBPool>,
    config: web::Data<AppConfig>,
    body:   web::Json<LoginRequest>,
) -> AppResult<HttpResponse> {
    let mut conn = pool.get().map_err(|e| AppError::Internal(e.to_string()))?;
    let username  = body.username.clone();
    let password  = body.password.clone();

    let user: User = web::block(move || {
        users::table
            .filter(users::username.eq(&username))
            .filter(users::status.eq(true))
            .select(User::as_select())
            .first(&mut conn)
            .optional()
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .map_err(AppError::Database)?
    .ok_or(AppError::Unauthorized)?;

    let valid = verify(password, user.encrypted_password.clone());

    if !valid {
        return Err(AppError::Unauthorized);
    }

    let token = create_token(
        user.id,
        user.username.clone(),
        user.role,
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

// POST /api/auth/register (admin only in production — open for setup)
#[post("/register")]
pub async fn register(
    pool: web::Data<DBPool>,
    body: web::Json<RegisterRequest>,
) -> AppResult<HttpResponse> {
    let mut conn = pool.get().map_err(|e| AppError::Internal(e.to_string()))?;

    let encrypted_password = password_hash(body.password.clone());

    let now = Utc::now().naive_utc();
    let role = body.role.unwrap_or(ROLE_OPERATOR);

    let new_user = NewUser {
        id:         Uuid::new_v4(),
        username:   body.username.clone(),
        email:      body.email.clone(),
        encrypted_password,
        role,
        status:     Some(true),
        created_at: now,
        updated_at: now,
        current_sign_in_at: None,
        last_sign_in_at: None,
        current_sign_in_ip: None,
        last_sign_in_ip: None,
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
        ) => AppError::Conflict("Username or email already exists".to_string()),
        other => AppError::Database(other),
    })?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "id":       user.id,
        "username": user.username,
        "email":    user.email,
        "role":     user.role,
    })))
}

// GET /api/auth/me
#[get("/me")]
pub async fn me(user: AuthUser) -> AppResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id":       user.claims().sub,
        "username": user.claims().username,
        "role":     user.claims().role,
    })))
}
