use diesel::QueryResult;
use uuid::Uuid;
use async_trait::async_trait;

use crate::models::slot::{NewSlot, Slot};

/// Trait defining the slot repository contract.
#[async_trait]
pub trait ISlotRepository: Send + Sync {
    async fn all(&self) -> QueryResult<Vec<Slot>>;
    async fn find(&self, id: &Uuid) -> QueryResult<Slot>;
    async fn create(&self, item: &NewSlot) -> QueryResult<Slot>;
    async fn update(&self, id: &Uuid, item: &NewSlot) -> QueryResult<Slot>;
    async fn destroy(&self, id: &Uuid) -> QueryResult<usize>;
}