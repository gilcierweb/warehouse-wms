use crate::{
    auth::password::{password_hash, verify},
    config::AppConfig,
    db::database::Database,
    errors::{AppError, AppResult},
    middleware::auth::{create_token, AuthUser},
    models::role::ROLE_OPERATOR,
    models::user::{NewUser, User},
    db::schema::users,
};
use actix_web::{web, HttpResponse, post, get, HttpRequest};
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_i18n::t;

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

#[derive(Debug, Deserialize)]
pub struct RecoverRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
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
    db:   web::Data<Database>,
    config: web::Data<AppConfig>,
    body:   web::Json<LoginRequest>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    println!("DEBUG: Login attempt for username: {}", body.username);
    
    let mut conn = db.pool.get().map_err(|e| AppError::Internal(e.to_string()))?;
    let username  = body.username.clone();
    let password  = body.password.clone();
    let username_for_error = username.clone(); // Clone for error message

    let user: User = web::block(move || {
        println!("DEBUG: Querying database for user with username: {}", username);
        let by_username = users::table
            .filter(users::username.eq(&username))
            .filter(users::status.eq(true))
            .select(User::as_select())
            .first(&mut conn)
            .optional()?;
            
        if by_username.is_some() {
            println!("DEBUG: User found by username");
            return Ok(by_username);
        }
        
        // If not found by username, try by email
        println!("DEBUG: User not found by username, trying by email");
        let by_email = users::table
            .filter(users::email.eq(&username))
            .filter(users::status.eq(true))
            .select(User::as_select())
            .first(&mut conn)
            .optional()?;
        
        if by_email.is_some() {
            println!("DEBUG: User found by email");
            return Ok(by_email);
        }
        
        println!("DEBUG: User not found by email");
        Err(diesel::result::Error::NotFound)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .map_err(AppError::Database)?
    .ok_or_else(|| {
        println!("DEBUG: User not found: {}", username_for_error);
        AppError::Unauthorized
    })?;

    println!("DEBUG: User found: {}, verifying password...", user.username);
    let valid = verify(password, user.encrypted_password.clone());
    println!("DEBUG: Password verification result: {}", valid);

    if !valid {
        return Err(AppError::Unauthorized);
    }

    // Extract client IP for login tracking
    let login_ip = req
        .connection_info()
        .realip_remote_addr()
        .map(|addr| addr.to_string())
        .or_else(|| req.peer_addr().map(|addr| addr.ip().to_string()))
        .unwrap_or_else(|| "unknown".to_string());

    let now = Utc::now().naive_utc();
    let user_id = user.id;

    // Update user login info in separate block
    let updated_user: User = web::block(move || {
        let mut conn = db.pool.get().map_err(|e| AppError::Internal(e.to_string()))?;
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set((
                users::current_sign_in_at.eq(Some(now)),
                users::last_sign_in_at.eq(user.current_sign_in_at),
                users::current_sign_in_ip.eq(Some(login_ip.clone())),
                users::last_sign_in_ip.eq(user.current_sign_in_ip),
                users::sign_in_count.eq(users::sign_in_count + 1),
            ))
            .returning(User::as_returning())
            .get_result(&mut conn)
            .map_err(|e| AppError::Internal(e.to_string()))
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))??;

    let token = create_token(
        updated_user.id,
        updated_user.username.clone(),
        updated_user.role,
        &config.jwt_secret,
        config.jwt_expiry_hours,
    )?;

    println!("DEBUG: Login successful for user: {}", updated_user.username);

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user_id:  updated_user.id,
        username: updated_user.username,
        role:     updated_user.role,
    }))
}

// POST /api/auth/register (admin only in production — open for setup)
#[post("/register")]
pub async fn register(
    db: web::Data<Database>,
    body: web::Json<RegisterRequest>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let mut conn = db.pool.get().map_err(|e| AppError::Internal(e.to_string()))?;

    let encrypted_password = password_hash(body.password.clone());

    let now = Utc::now().naive_utc();
    let role = body.role.unwrap_or(ROLE_OPERATOR.as_i32());

    // Extract client IP
    let ip = req
        .connection_info()
        .realip_remote_addr()
        .map(|addr| addr.to_string())
        .or_else(|| req.peer_addr().map(|addr| addr.ip().to_string()))
        .unwrap_or_else(|| "unknown".to_string());

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
        current_sign_in_ip: Some(ip.clone()),
        last_sign_in_ip: Some(ip),
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
        ) => AppError::Conflict(t!("auth.register.username_exists").to_string()),
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

// POST /api/auth/recover
#[post("/recover")]
pub async fn recover_password(
    db: web::Data<Database>,
    body: web::Json<RecoverRequest>,
) -> AppResult<HttpResponse> {
    let email = body.email.clone();
    let db_clone = db.clone();

    let _user: Option<User> = web::block(move || {
        let mut conn = db_clone.pool.get().unwrap();
        users::table
            .filter(users::email.eq(&email))
            .select(User::as_select())
            .first(&mut conn)
            .optional()
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .map_err(AppError::Database)?;

    let token = Uuid::new_v4().to_string();
    let now = Utc::now().naive_utc();
    let mut response_token = String::new();

    if let Some(user) = _user {
        let token_clone = token.clone();
        let db_clone2 = db.clone();
        web::block(move || {
            let mut conn = db_clone2.pool.get().unwrap();
            diesel::update(users::table.filter(users::id.eq(user.id)))
                .set((
                    users::reset_password_token.eq(&token_clone),
                    users::reset_password_sent_at.eq(&now),
                ))
                .execute(&mut conn)
        })
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(AppError::Database)?;

        println!("DEBUG: Password recovery token generated for user: {}", user.username);
        response_token = token;
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": t!("auth.recover.email_sent").to_string(),
        "token": response_token
    })))
}

// POST /api/auth/reset
#[post("/reset")]
pub async fn reset_password(
    db: web::Data<Database>,
    body: web::Json<ResetPasswordRequest>,
) -> AppResult<HttpResponse> {
    let token = body.token.clone();
    let new_password = body.password.clone();
    let db_clone = db.clone();

    let user: User = web::block(move || {
        let mut conn = db_clone.pool.get().unwrap();
        users::table
            .filter(users::reset_password_token.eq(&token))
            .select(User::as_select())
            .first(&mut conn)
            .optional()
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .map_err(AppError::Database)?
    .ok_or(AppError::BadRequest(t!("auth.reset.token_invalid").to_string()))?;

    let encrypted_password = password_hash(new_password);
    let now = Utc::now().naive_utc();

    web::block(move || {
        let mut conn = db.pool.get().unwrap();
        diesel::update(users::table.filter(users::id.eq(user.id)))
            .set((
                users::encrypted_password.eq(encrypted_password),
                users::reset_password_token.eq::<Option<String>>(None),
                users::updated_at.eq(now),
            ))
            .execute(&mut conn)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .map_err(AppError::Database)?;

    println!("DEBUG: Password reset successful for user: {}", user.username);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": t!("auth.reset.success").to_string()
    })))
}
