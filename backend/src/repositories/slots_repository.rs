use diesel::QueryResult;
use uuid::Uuid;
use async_trait::async_trait;

use crate::models::slot::{NewSlot, Slot};
use crate::repositories::traits::IRepository;

/// Trait defining the slot repository contract.
#[async_trait]
pub trait ISlotRepository: IRepository<Slot, NewSlot> + Send + Sync {}