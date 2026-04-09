use std::sync::Arc;
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::models::user_key::{KeyType, NewUserKey};
use crate::repositories::AppContainer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UploadKeysRequest {
    pub identity_key: String,
    pub signed_prekey: String,
    pub signed_signature: String,
    pub one_time_prekeys: Vec<String>,
}

#[derive(Serialize)]
pub struct KeyBundleResponse {
    pub identity_key: String,
    pub signed_prekey: String,
    pub signed_signature: String,
    pub one_time_prekey: Option<String>,
}

pub struct E2eKeysService;

impl E2eKeysService {
    pub async fn upload_keys(
        container: &Arc<AppContainer>,
        user_id: Uuid,
        req: UploadKeysRequest,
    ) -> AppResult<()> {
        let mut keys_to_insert = Vec::new();

        // 1. Identity Key
        keys_to_insert.push(NewUserKey::new(
            user_id,
            KeyType::Identity.as_i32(),
            req.identity_key,
            None,
        ));

        // 2. Signed PreKey
        keys_to_insert.push(NewUserKey::new(
            user_id,
            KeyType::SignedPrekey.as_i32(),
            req.signed_prekey,
            Some(req.signed_signature),
        ));

        // 3. One-Time PreKeys
        for otk in req.one_time_prekeys {
            keys_to_insert.push(NewUserKey::new(
                user_id,
                KeyType::OneTimePrekey.as_i32(),
                otk,
                None, // OTKs usually aren't signed individually in Signal protocol, they are just validated via the session
            ));
        }

        // We use a batch insert via the repository
        container
            .user_keys
            .insert_keys(keys_to_insert)
            .await
            .map_err(|e| {
                tracing::error!("Failed to insert user keys: {:?}", e);
                AppError::Database(e)
            })?;

        Ok(())
    }

    pub async fn fetch_bundle(
        container: &Arc<AppContainer>,
        target_user_id: Uuid,
    ) -> AppResult<KeyBundleResponse> {
        let bundle_res = container.user_keys.fetch_key_bundle(target_user_id).await
            .map_err(|e| {
                tracing::error!("Failed to fetch key bundle: {:?}", e);
                AppError::Database(e)
            })?;

        let (identity, signed, one_time) = bundle_res.ok_or_else(|| {
            AppError::NotFound("Target user has no keys registered".to_string())
        })?;

        Ok(KeyBundleResponse {
            identity_key: identity.public_key,
            signed_prekey: signed.public_key,
            // The signature field must exist, if not, it violates logic, but we default gracefully
            signed_signature: signed.signature.unwrap_or_default(),
            one_time_prekey: one_time.map(|k| k.public_key),
        })
    }
}
