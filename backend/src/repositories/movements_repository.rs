use diesel::QueryResult;
use uuid::Uuid;
use async_trait::async_trait;

use crate::models::movement::{Movement, NewMovement};
use crate::repositories::traits::IRepository;

/// Trait defining the movement repository contract.
#[async_trait]
pub trait IMovementRepository: IRepository<Movement, NewMovement> + Send + Sync {}