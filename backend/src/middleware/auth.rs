use crate::errors::AppError;
use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::role::{UserRole, ROLE_ADMIN};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub username: String,
    pub role: i32,
    pub exp: usize,
}

/// Extractor: `AuthUser` in handler params → validates JWT automatically
#[derive(Debug, Clone)]
pub struct AuthUser(pub Claims);

impl AuthUser {
    pub fn claims(&self) -> &Claims {
        &self.0
    }

    pub fn require_role(&self, role: UserRole) -> Result<(), AppError> {
        let role_i32 = role.as_i32();
        if self.0.role == ROLE_ADMIN.as_i32() || self.0.role == role_i32 {
            Ok(())
        } else {
            Err(AppError::Forbidden(format!(
                "Required role '{:?}', you have '{}'",
                role, self.0.role
            )))
        }
    }

    pub fn role(&self) -> UserRole {
        UserRole::try_from(self.0.role).unwrap_or(UserRole::Operator)
    }
}

impl FromRequest for AuthUser {
    type Error = AppError;
    type Future = Ready<Result<Self, AppError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let result = extract_claims(req);
        ready(result.map(AuthUser))
    }
}

fn extract_claims(req: &HttpRequest) -> Result<Claims, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let secret = req
        .app_data::<web::Data<crate::config::AppConfig>>()
        .map(|c| c.jwt_secret.clone())
        .unwrap_or_default();

    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(token, &key, &validation)
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized)
}

pub fn create_token(
    user_id: Uuid,
    username: String,
    role: i32,
    secret: &str,
    expiry_h: i64,
) -> Result<String, AppError> {
    use chrono::Utc;
    use jsonwebtoken::{encode, EncodingKey, Header};

    let exp = (Utc::now() + chrono::Duration::hours(expiry_h)).timestamp() as usize;
    let claims = Claims {
        sub: user_id,
        username,
        role,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(e.to_string()))
}
