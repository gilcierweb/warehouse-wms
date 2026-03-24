use diesel::QueryResult;
use uuid::Uuid;
use async_trait::async_trait;

use crate::models::user::{NewUser, User};

/// Trait defining the user repository contract.
/// Any implementation (Diesel, Mock, etc.) must satisfy this interface.
#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn all(&self) -> QueryResult<Vec<User>>;
    async fn find(&self, id: &Uuid) -> QueryResult<User>;
    async fn create(&self, item: &NewUser) -> QueryResult<User>;
    async fn update(&self, id: &Uuid, item: &NewUser) -> QueryResult<User>;
    async fn destroy(&self, id: &Uuid) -> QueryResult<usize>;
}