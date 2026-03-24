use diesel::QueryResult;
use uuid::Uuid;
use async_trait::async_trait;

use crate::models::profile::{NewProfile, Profile};

/// Trait defining the profile repository contract.
#[async_trait]
pub trait IProfileRepository: Send + Sync {
    async fn all(&self) -> QueryResult<Vec<Profile>>;
    async fn find(&self, id: &Uuid) -> QueryResult<Profile>;
    async fn create(&self, item: &NewProfile) -> QueryResult<Profile>;
    async fn update(&self, id: &Uuid, item: &NewProfile) -> QueryResult<Profile>;
    async fn destroy(&self, id: &Uuid) -> QueryResult<usize>;
}