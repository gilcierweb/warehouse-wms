use crate::{
    auth::password::{password_hash, verify},
    config::app_config::AppConfig,
    errors::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::profile::NewProfile,
    models::refresh_token::{NewRefreshToken, RefreshToken},
    models::user::{NewUser, User},
    repositories::container::AppContainer,
};
use actix_web::{HttpRequest, HttpResponse, get, post, web, cookie::{Cookie, SameSite, time::Duration as CookieDuration}};
use chrono::{Duration, Utc};
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

const PATH_COOKIE_HTTP_ONLY: &str = "/api";
// --Request/Response DTOs

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email(message = "auth.validation.invalid_email"))]
    pub email: String,
    #[validate(length(min = 8, message = "auth.validation.password_too_short"))]
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email(message = "auth.validation.invalid_email"))]
    pub email: String,
    pub password: String,
    /// Optional TOTP code if 2FA is enabled
    pub otp_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RecoverRequest {
    #[validate(email(message = "auth.validation.invalid_email"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,
    #[validate(length(min = 8, message = "auth.validation.password_too_short"))]
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Deserialize)]
pub struct Enable2FARequest {
    pub otp_code: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    #[validate(length(min = 8, message = "auth.validation.password_too_short"))]
    pub new_password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub profile_id: Uuid,
    pub is_otp_enabled: bool,
    pub roles: Vec<String>,
}

// POST /api/auth/register
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully"),
        (status = 409, description = "Email already exists")
    )
)]
#[post("/register")]
pub async fn register(
    container: web::Data<AppContainer>,
    body: web::Json<RegisterRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    if body.password != body.password_confirmation {
        return Err(AppError::Validation(
            t!("auth.password.mismatch").into_owned(),
        ));
    }

    let encrypted_password = password_hash(body.password.clone());
    let now = Utc::now();
    let confirmation_token = Uuid::new_v4().to_string();

    let new_user = NewUser {
        id: Uuid::new_v4(),
        email: body.email.clone(),
        password_hash: encrypted_password,
        confirmation_token: Some(confirmation_token.clone()),
        created_at: now,
        updated_at: now,
    };

    let user: User = match container.users.create(&new_user).await {
        Ok(u) => u,
        Err(DieselError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
            return Err(AppError::Conflict(
                t!("auth.register.email_exists").into_owned(),
            ));
        }
        Err(e) => return Err(AppError::Database(e)),
    };

    // Create profile for the user
    let new_profile = NewProfile::for_user(user.id);
    container
        .profiles
        .create(&new_profile)
        .await
        .map_err(AppError::Database)?;

    if let Err(err) = send_confirmation_email(&container.config, &user.email, &confirmation_token).await {
        tracing::error!("failed to send confirmation email: {err}");
    }

    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": t!("auth.register.success"),
        "user_id": user.id,
    })))
}

// GET /api/auth/confirm?token=xxx
#[get("/confirm")]
pub async fn confirm(
    container: web::Data<AppContainer>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> AppResult<HttpResponse> {
    let token = query
        .get("token")
        .ok_or_else(|| AppError::BadRequest(t!("auth.reset.token_invalid").into_owned()))?;

    // Find user by confirmation token and confirm email
    let updated = container
        .users
        .confirm_email(token)
        .await
        .map_err(AppError::Database)?;
    if updated == 0 {
        return Err(AppError::BadRequest(
            t!("auth.reset.token_invalid").into_owned(),
        ));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": t!("auth.register.email_confirmed")
    })))
}

// POST /api/auth/login
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Unauthorized - Invalid credentials"),
        (status = 403, description = "Forbidden - OTP required"),
        (status = 423, description = "Locked - Temporary lockout due to failed attempts")
    )
)]
#[post("/login")]
pub async fn login(
    req: HttpRequest,
    container: web::Data<AppContainer>,
    body: web::Json<LoginRequest>,
) -> AppResult<HttpResponse> {
    const LOCKOUT_MINUTES: i64 = 15;

    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Find user by email
    let user: User = match container.users.find_by_email(&body.email).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return Err(AppError::Unauthorized(t!("auth.login.failed").into_owned()));
        }
        Err(e) => return Err(AppError::Database(e)),
    };

    // Check lockout window and auto-clear expired lockout state.
    if let Some(locked_at) = user.locked_at {
        if Utc::now() < locked_at + Duration::minutes(LOCKOUT_MINUTES) {
            return Err(AppError::BadRequest(t!("auth.login.locked").into_owned()));
        }

        container
            .users
            .clear_lockout(&user.id)
            .await
            .map_err(AppError::Database)?;
    }

    // Check email confirmation
    if !user.is_confirmed() {
        return Err(AppError::BadRequest(
            t!("auth.login.email_not_confirmed").into_owned(),
        ));
    }

    // Verify password
    let password_valid = verify(body.password.clone(), user.password_hash.clone());
    if !password_valid {
        container
            .users
            .record_failed_login(&user.id, 10)
            .await
            .map_err(AppError::Database)?;
        return Err(AppError::Unauthorized(t!("auth.login.failed").into_owned()));
    }

    // Verify TOTP if 2FA is enabled
    if user.is_otp_enabled() {
        match &body.otp_code {
            None => {
                return Ok(HttpResponse::Ok().json(serde_json::json!({
                    "requires_otp": true,
                    "message": t!("auth.2fa.setup_required")
                })));
            }
            Some(code) => {
                let secret = user.totp_secret.as_ref().ok_or(AppError::Internal(
                    t!("auth.2fa.invalid_secret").into_owned(),
                ))?;
                verify_totp(secret, code)?;
            }
        }
    }

    // Get profile ID
    let profile = container
        .profiles
        .find_by_user_id(&user.id)
        .await
        .map_err(AppError::Database)?
        .ok_or(AppError::Internal(
            t!("users.profile_not_found").into_owned(),
        ))?;

    // Get user roles
    let roles = container
        .users
        .get_user_roles(&user.id)
        .await
        .map_err(AppError::Database)?;

    // Generate tokens
    let access_token = crate::middleware::auth::create_token(
        user.id,
        profile.id,
        user.email.clone(),
        roles.clone(),
        &container.config.jwt_secret,
        container.config.jwt_access_expiry_secs,
    )?;

    let refresh_token_plain = generate_random_token(48);
    let refresh_token_hash = hash_token(&refresh_token_plain);

    // Store refresh token
    let ip_string = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    let ip: Option<ipnet::IpNet> = ip_string
        .as_ref()
        .and_then(|s| s.parse::<ipnet::IpNet>().ok());

    let new_refresh = NewRefreshToken {
        id: Uuid::new_v4(),
        user_id: user.id,
        token_hash: refresh_token_hash,
        device_info: req
            .headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string()),
        ip_address: ip_string.clone(),
        expires_at: Utc::now()
            + chrono::Duration::seconds(container.config.jwt_refresh_expiry_secs),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    container
        .refresh_tokens
        .create(&new_refresh)
        .await
        .map_err(AppError::Database)?;

    // Record successful login
    container
        .users
        .record_successful_login(&user.id, ip)
        .await
        .map_err(AppError::Database)?;

    // Create HttpOnly cookie for the refresh token
    // Path /api para incluir /api/v1/auth e /api/proxy
    let mut cookie = Cookie::build("refresh_token", refresh_token_plain.clone())
        .path(PATH_COOKIE_HTTP_ONLY)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(CookieDuration::seconds(container.config.jwt_refresh_expiry_secs as i64))
        .finish();

    // Secure flag only in production (if using HTTPS)
    if container.config.is_production() {
        cookie.set_secure(true);
    }

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(AuthResponse {
            access_token,
            refresh_token: refresh_token_plain,
            token_type: "Bearer",
            expires_in: container.config.jwt_access_expiry_secs,
            user: UserInfo {
                id: user.id,
                email: user.email.clone(),
                profile_id: profile.id,
                is_otp_enabled: user.is_otp_enabled(),
                roles,
            },
        }))
}

// POST /api/auth/refresh
#[post("/refresh")]
pub async fn refresh(
    req: HttpRequest,
    container: web::Data<AppContainer>,
    body: Option<web::Json<RefreshRequest>>,
) -> AppResult<HttpResponse> {
    let body_refresh_token = body.and_then(|b| b.into_inner().refresh_token);

    // 1. Try to get token from HttpOnly cookie first
    // 2. Fallback to JSON body for non-browser clients
    let refresh_token = req.cookie("refresh_token")
        .map(|c| c.value().to_string())
        .or(body_refresh_token)
        .ok_or_else(|| AppError::Unauthorized(t!("auth.refresh.invalid_token").into_owned()))?;

    let token_hash = hash_token(&refresh_token);

    let stored: RefreshToken = match container
        .refresh_tokens
        .find_by_token_hash(&token_hash)
        .await
    {
        Ok(Some(t)) => t,
        Ok(None) => return Err(AppError::Unauthorized(t!("auth.refresh.invalid_token").into_owned())),
        Err(e) => return Err(AppError::Database(e)),
    };

    if !stored.is_valid() {
        return Err(AppError::Unauthorized(
            t!("auth.refresh.token_expired").into_owned(),
        ));
    }

    // Revoke old token
    container
        .refresh_tokens
        .revoke(&stored.id)
        .await
        .map_err(AppError::Database)?;

    // Get user
    let user = container
        .users
        .find(&stored.user_id)
        .await
        .map_err(AppError::Database)?;

    // Get profile
    let _profile = container
        .profiles
        .find_by_user_id(&user.id)
        .await
        .map_err(AppError::Database)?
        .ok_or(AppError::Internal(
            t!("users.profile_not_found").into_owned(),
        ))?;

    let roles = container
        .users
        .get_user_roles(&user.id)
        .await
        .map_err(AppError::Database)?;

    // Generate new tokens
    let access_token = crate::middleware::auth::create_token(
        user.id,
        _profile.id,
        user.email.clone(),
        roles,
        &container.config.jwt_secret,
        container.config.jwt_access_expiry_secs,
    )?;

    let new_refresh_plain = generate_random_token(48);
    let new_refresh = NewRefreshToken {
        id: Uuid::new_v4(),
        user_id: user.id,
        token_hash: hash_token(&new_refresh_plain),
        device_info: stored.device_info,
        ip_address: stored.ip_address,
        expires_at: Utc::now()
            + chrono::Duration::seconds(container.config.jwt_refresh_expiry_secs),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    container
        .refresh_tokens
        .create(&new_refresh)
        .await
        .map_err(AppError::Database)?;

    // Refresh cookie - path /api para incluir /api/proxy
    let mut cookie = Cookie::build("refresh_token", new_refresh_plain.clone())
        .path(PATH_COOKIE_HTTP_ONLY)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(CookieDuration::seconds(container.config.jwt_refresh_expiry_secs as i64))
        .finish();

    if container.config.is_production() {
        cookie.set_secure(true);
    }

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({
            "access_token": access_token,
            "refresh_token": new_refresh_plain,
            "token_type": "Bearer",
            "expires_in": container.config.jwt_access_expiry_secs,
        })))
}

// POST /api/auth/logout
#[post("/logout")]
pub async fn logout(
    req: HttpRequest,
    container: web::Data<AppContainer>,
    body: Option<web::Json<RefreshRequest>>,
) -> AppResult<HttpResponse> {
    let body_refresh_token = body.and_then(|b| b.into_inner().refresh_token);

    let refresh_token = req.cookie("refresh_token")
        .map(|c| c.value().to_string())
        .or(body_refresh_token);

    if let Some(token_val) = refresh_token {
        let token_hash = hash_token(&token_val);

        if let Ok(Some(token)) = container
            .refresh_tokens
            .find_by_token_hash(&token_hash)
            .await
        {
            container
                .refresh_tokens
                .revoke(&token.id)
                .await
                .map_err(AppError::Database)?;
        }
    }

    // Clear the HttpOnly cookie - path /api para incluir /api/proxy
    let cookie = Cookie::build("refresh_token", "")
        .path(PATH_COOKIE_HTTP_ONLY)
        .max_age(CookieDuration::seconds(0))
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({
            "message": t!("auth.logout.success")
        })))
}

// POST /api/auth/recover
#[post("/recover")]
pub async fn recover_password(
    container: web::Data<AppContainer>,
    body: web::Json<RecoverRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Ignore errors — always return success to prevent email enumeration
    if let Ok(Some(user)) = container.users.find_by_email(&body.email).await {
        let token = Uuid::new_v4().to_string();
        let now = Utc::now().naive_utc();

        container
            .users
            .create_password_reset_token(&user.id, &token, now)
            .await
            .map_err(AppError::Database)?;

        if let Err(err) = send_password_reset_email(&container.config, &user.email, &token).await {
            tracing::error!("failed to send password reset email: {err}");
        }
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": t!("auth.recover.email_sent")
    })))
}

// POST /api/auth/reset
#[post("/reset")]
pub async fn reset_password(
    container: web::Data<AppContainer>,
    body: web::Json<ResetPasswordRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    if body.password != body.password_confirmation {
        return Err(AppError::Validation(t!("auth.password.mismatch").into_owned()));
    }

    let user = container
        .users
        .find_by_reset_token(&body.token)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::BadRequest(t!("auth.reset.token_invalid").into_owned()))?;

    // Always persist a hashed password, never plaintext.
    let hashed_password = password_hash(body.password.clone());

    let updated = container
        .users
        .reset_password(&body.token, &hashed_password)
        .await
        .map_err(AppError::Database)?;
    if updated == 0 {
        return Err(AppError::BadRequest(
            t!("auth.reset.token_invalid").into_owned(),
        ));
    }

    container
        .refresh_tokens
        .revoke_all_for_user(&user.id)
        .await
        .map_err(AppError::Database)?;

    let clear_cookie = Cookie::build("refresh_token", "")
        .path(PATH_COOKIE_HTTP_ONLY)
        .max_age(CookieDuration::seconds(0))
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(clear_cookie)
        .json(serde_json::json!({
            "message": t!("auth.reset.success")
        })))
}

// POST /api/auth/2fa/setup
#[post("/2fa/setup")]
pub async fn setup_2fa(
    user: AuthUser,
    container: web::Data<AppContainer>,
) -> AppResult<HttpResponse> {
    use totp_rs::{Algorithm as TotpAlgorithm, Secret, TOTP};

    let secret = Secret::generate_secret();
    let secret_base32 = secret.to_encoded().to_string();

    let totp = TOTP::new(
        TotpAlgorithm::SHA1,
        6,
        1,
        30,
        secret.to_bytes().unwrap(),
        Some(container.config.totp_issuer.clone()),
        user.claims().email.clone(),
    )
    .map_err(|e| AppError::Internal(t!("auth.2fa.setup_error", error = e).into_owned()))?;

    let qr_code_url = totp.get_url();

    // Store secret temporarily (not enabled until verified)
    container
        .users
        .set_otp_secret(&user.claims().sub, &secret_base32)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "secret": secret_base32,
        "qr_code_url": qr_code_url,
        "message": t!("auth.2fa.setup_instructions")
    })))
}

// POST /api/auth/2fa/enable
#[post("/2fa/enable")]
pub async fn enable_2fa(
    user: AuthUser,
    container: web::Data<AppContainer>,
    body: web::Json<Enable2FARequest>,
) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;

    let user_data = container
        .users
        .find(&user_id)
        .await
        .map_err(AppError::Database)?;

    let secret = user_data
        .totp_secret
        .as_ref()
        .ok_or_else(|| AppError::BadRequest(t!("auth.2fa.setup_not_initiated").into_owned()))?;

    verify_totp(secret, &body.otp_code)?;

    // Generate backup codes
    let backup_codes: Vec<String> = (0..8)
        .map(|_| generate_random_token(4).to_uppercase())
        .collect();

    container
        .users
        .enable_2fa(&user_id, &backup_codes)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": t!("auth.2fa.setup_success"),
        "backup_codes": backup_codes,
        "warning": t!("auth.2fa.backup_codes_warning")
    })))
}

// POST /api/auth/2fa/disable
#[post("/2fa/disable")]
pub async fn disable_2fa(
    user: AuthUser,
    container: web::Data<AppContainer>,
    body: web::Json<Enable2FARequest>,
) -> AppResult<HttpResponse> {
    let user_id = user.claims().sub;

    let user_data = container
        .users
        .find(&user_id)
        .await
        .map_err(AppError::Database)?;

    let secret = user_data
        .totp_secret
        .as_ref()
        .ok_or_else(|| AppError::BadRequest(t!("auth.2fa.not_enabled").into_owned()))?;

    verify_totp(secret, &body.otp_code)?;

    container
        .users
        .disable_2fa(&user_id)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": t!("auth.2fa.disabled_success")
    })))
}

// POST /api/auth/change-password
#[post("/change-password")]
pub async fn change_password(
    user: AuthUser,
    container: web::Data<AppContainer>,
    body: web::Json<ChangePasswordRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    if body.new_password != body.password_confirmation {
        return Err(AppError::Validation(t!("auth.password.mismatch").into_owned()));
    }

    let user_id = user.claims().sub;

    let user_data = container
        .users
        .find(&user_id)
        .await
        .map_err(AppError::Database)?;

    if !verify(
        body.current_password.clone(),
        user_data.password_hash.clone(),
    ) {
        return Err(AppError::Unauthorized(
            t!("auth.password.invalid_current").into_owned(),
        ));
    }

    validate_password_strength(&body.new_password)?;
    let hashed = password_hash(body.new_password.clone());

    container
        .users
        .update_password(&user_id, &hashed)
        .await
        .map_err(AppError::Database)?;

    container
        .refresh_tokens
        .revoke_all_for_user(&user_id)
        .await
        .map_err(AppError::Database)?;

    let clear_cookie = Cookie::build("refresh_token", "")
        .path(PATH_COOKIE_HTTP_ONLY)
        .max_age(CookieDuration::seconds(0))
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(clear_cookie)
        .json(serde_json::json!({
            "message": t!("auth.password.changed_success")
        })))
}

// GET /api/auth/me
#[get("/me")]
pub async fn me(user: AuthUser) -> AppResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": user.claims().sub,
        "email": user.claims().email,
    })))
}

// ── Helpers ────────────────────────────────────────────────────────

fn verify_totp(secret_base32: &str, code: &str) -> AppResult<()> {
    use totp_rs::{Algorithm as TotpAlgorithm, Secret, TOTP};

    let secret = Secret::Encoded(secret_base32.to_string());
    let totp = TOTP::new(
        TotpAlgorithm::SHA1,
        6,
        1,
        30,
        secret
            .to_bytes()
            .map_err(|_| AppError::Unauthorized(t!("auth.2fa.invalid_secret").into_owned()))?,
        None,
        "".to_string(),
    )
    .map_err(|_| AppError::Unauthorized(t!("auth.2fa.invalid_totp").into_owned()))?;

    if totp
        .check_current(code)
        .map_err(|_| AppError::Unauthorized(t!("auth.2fa.invalid_code").into_owned()))?
    {
        Ok(())
    } else {
        Err(AppError::Unauthorized(t!("auth.2fa.invalid_code").into_owned()))
    }
}

fn generate_random_token(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn hash_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn validate_password_strength(password: &str) -> AppResult<()> {
    if password.len() < 8 {
        return Err(AppError::Validation(
            t!("auth.password.too_short").into_owned(),
        ));
    }
    Ok(())
}

#[derive(Serialize)]
struct ResendEmailRequest {
    from: String,
    to: Vec<String>,
    subject: String,
    html: String,
}

async fn send_confirmation_email(config: &AppConfig, to_email: &str, token: &str) -> AppResult<()> {
    if config.resend_api_key.trim().is_empty() || config.email_from.trim().is_empty() {
        tracing::warn!("RESEND_API_KEY/EMAIL_FROM not configured; skipping confirmation email");
        return Ok(());
    }

    let frontend_base = config.frontend_url.trim_end_matches('/');
    let confirm_url = format!(
        "{frontend_base}/auth/confirm?token={}",
        urlencoding::encode(token)
    );
    let from = if config.email_from_name.trim().is_empty() {
        config.email_from.clone()
    } else {
        format!("{} <{}>", config.email_from_name, config.email_from)
    };
    let payload = ResendEmailRequest {
        from,
        to: vec![to_email.to_string()],
        subject: "Confirme seu email".to_string(),
        html: format!(
            "<p>Bem-vindo(a)! Clique no link abaixo para confirmar seu email:</p>\
             <p><a href=\"{confirm_url}\">Confirmar email</a></p>"
        ),
    };

    let response = reqwest::Client::new()
        .post("https://api.resend.com/emails")
        .bearer_auth(&config.resend_api_key)
        .json(&payload)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("failed to call email provider: {e}")))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let body = response.text().await.unwrap_or_else(|_| "".to_string());
        Err(AppError::Internal(format!(
            "email provider returned non-success status: {body}"
        )))
    }
}

async fn send_password_reset_email(
    config: &AppConfig,
    to_email: &str,
    token: &str,
) -> AppResult<()> {
    if config.resend_api_key.trim().is_empty() || config.email_from.trim().is_empty() {
        tracing::warn!("RESEND_API_KEY/EMAIL_FROM not configured; skipping password reset email");
        return Ok(());
    }

    let frontend_base = config.frontend_url.trim_end_matches('/');
    let reset_url = format!(
        "{frontend_base}/auth/reset-password?token={}",
        urlencoding::encode(token)
    );
    let from = if config.email_from_name.trim().is_empty() {
        config.email_from.clone()
    } else {
        format!("{} <{}>", config.email_from_name, config.email_from)
    };
    let payload = ResendEmailRequest {
        from,
        to: vec![to_email.to_string()],
        subject: "Redefinicao de senha".to_string(),
        html: format!(
            "<p>Recebemos um pedido para redefinir sua senha.</p>\
             <p><a href=\"{reset_url}\">Clique aqui para redefinir sua senha</a></p>\
             <p>Este link expira em 1 hora.</p>"
        ),
    };

    let response = reqwest::Client::new()
        .post("https://api.resend.com/emails")
        .bearer_auth(&config.resend_api_key)
        .json(&payload)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("failed to call email provider: {e}")))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let body = response.text().await.unwrap_or_else(|_| "".to_string());
        Err(AppError::Internal(format!(
            "email provider returned non-success status: {body}"
        )))
    }
}

// Helper trait extensions for User
pub trait UserExt {
    fn is_locked(&self) -> bool;
    fn is_confirmed(&self) -> bool;
    fn is_otp_enabled(&self) -> bool;
}

impl UserExt for User {
    fn is_locked(&self) -> bool {
        self.locked_at.is_some()
    }

    fn is_confirmed(&self) -> bool {
        self.confirmed_at.is_some()
    }

    fn is_otp_enabled(&self) -> bool {
        self.totp_enabled && self.totp_secret.is_some()
    }
}

// Tests temporarily disabled - test_utils module not available
// #[cfg(test)]
// mod tests {
//     // Test code requires test_utils module
// }
