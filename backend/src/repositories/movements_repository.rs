use diesel::QueryResult;
use uuid::Uuid;
use async_trait::async_trait;

use crate::models::movement::{Movement, NewMovement};

/// Trait defining the movement repository contract.
#[async_trait]
pub trait IMovementRepository: Send + Sync {
    async fn all(&self) -> QueryResult<Vec<Movement>>;
    async fn find(&self, id: &Uuid) -> QueryResult<Movement>;
    async fn create(&self, item: &NewMovement) -> QueryResult<Movement>;
    async fn update(&self, id: &Uuid, item: &NewMovement) -> QueryResult<Movement>;
    async fn destroy(&self, id: &Uuid) -> QueryResult<usize>;
}