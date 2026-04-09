use crate::config::AppConfig;
pub use crate::middleware::auth::Claims;
use jsonwebtoken::{DecodingKey, Validation, decode};

pub fn verify_access_token(
    token: &str,
    config: &AppConfig,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}
