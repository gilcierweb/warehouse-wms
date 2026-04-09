#![allow(dead_code)]

use async_trait::async_trait;
use diesel::QueryResult;
use uuid::Uuid;

use crate::models::profile::{NewProfile, Profile};

/// Trait defining the profile repository contract.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IProfileRepository: Send + Sync {
    async fn all(&self) -> QueryResult<Vec<Profile>>;
    async fn find(&self, id: &Uuid) -> QueryResult<Profile>;
    async fn create(&self, item: &NewProfile) -> QueryResult<Profile>;
    async fn update(&self, id: &Uuid, item: &NewProfile) -> QueryResult<Profile>;
    async fn destroy(&self, id: &Uuid) -> QueryResult<usize>;

    /// Find profile by user ID
    async fn find_by_user_id(&self, user_id: &Uuid) -> QueryResult<Option<Profile>>;
}
