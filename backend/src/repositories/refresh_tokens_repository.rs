#![allow(dead_code)]

use async_trait::async_trait;
use diesel::QueryResult;
use uuid::Uuid;

use crate::models::refresh_token::{NewRefreshToken, RefreshToken};

/// Trait defining the refresh token repository contract.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IRefreshTokenRepository: Send + Sync {
    async fn all(&self) -> QueryResult<Vec<RefreshToken>>;
    async fn find(&self, id: &Uuid) -> QueryResult<RefreshToken>;
    async fn create(&self, item: &NewRefreshToken) -> QueryResult<RefreshToken>;
    async fn update(&self, id: &Uuid, item: &NewRefreshToken) -> QueryResult<RefreshToken>;
    async fn destroy(&self, id: &Uuid) -> QueryResult<usize>;

    /// Find refresh token by its hash
    async fn find_by_token_hash(&self, token_hash: &str) -> QueryResult<Option<RefreshToken>>;

    /// Revoke a refresh token
    async fn revoke(&self, id: &Uuid) -> QueryResult<usize>;

    /// Revoke all tokens for a user
    async fn revoke_all_for_user(&self, user_id: &Uuid) -> QueryResult<usize>;
}

crate::impl_repository_for_trait!(IRefreshTokenRepository, RefreshToken, NewRefreshToken);
