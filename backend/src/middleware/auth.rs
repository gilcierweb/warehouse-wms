use actix_web::{FromRequest, http::header::AUTHORIZATION};
use std::future::{Ready, ready};

use crate::{
    AppState,
    errors::{AppError, AppResult},
};

/// JWT Claims structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub profile_id: uuid::Uuid,
    pub email: String,
    pub role: i32,
    #[serde(default)]
    pub roles: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }

    pub fn profile_id(&self) -> Option<uuid::Uuid> {
        Some(self.profile_id)
    }
}

/// Extractor for authenticated user claims
/// Reads and validates JWT token directly from Authorization header
pub struct AuthUser {
    claims: Claims,
}

impl AuthUser {
    pub fn claims(&self) -> &Claims {
        &self.claims
    }
}

impl FromRequest for AuthUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        // Extract Bearer token from Authorization header
        let token = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));

        match token {
            None => {
                eprintln!("[AuthUser] No Bearer token found in Authorization header");
                ready(Err(AppError::Unauthorized(
                    t!("errors.missing_auth_header").into_owned(),
                )))
            }
            Some(t) => {
                // Get JWT secret from AppState
                let state = req.app_data::<actix_web::web::Data<AppState>>();
                let secret = state
                    .as_ref()
                    .map(|s| s.config.jwt_secret.clone())
                    .unwrap_or_default();

                eprintln!(
                    "[AuthUser] Token found, state present: {}, secret length: {}",
                    state.is_some(),
                    secret.len()
                );

                match verify_token(t, &secret) {
                    Ok(claims) => {
                        eprintln!(
                            "[AuthUser] Token verified successfully for user: {}",
                            claims.sub
                        );
                        ready(Ok(AuthUser { claims }))
                    }
                    Err(e) => {
                        eprintln!("[AuthUser] Token verification failed: {:?}", e);
                        ready(Err(e))
                    }
                }
            }
        }
    }
}

/// Create a JWT token for a user
pub fn create_token(
    user_id: uuid::Uuid,
    profile_id: uuid::Uuid,
    email: String,
    roles: Vec<String>,
    jwt_secret: &str,
    expiry_secs: i64,
) -> AppResult<String> {
    use chrono::Utc;
    use jsonwebtoken::{EncodingKey, Header, encode};

    let now = Utc::now();
    let exp = (now + chrono::Duration::seconds(expiry_secs)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        profile_id,
        email,
        role: 0,
        roles,
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(t!("ws.token_creation_failed", error = e).into_owned()))
}

/// Verify a JWT token
pub fn verify_token(token: &str, jwt_secret: &str) -> AppResult<Claims> {
    use jsonwebtoken::{DecodingKey, Validation, decode};

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized(t!("errors.invalid_token").into_owned()))
}
