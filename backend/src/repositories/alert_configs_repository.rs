use diesel::QueryResult;
use uuid::Uuid;
use async_trait::async_trait;

use crate::models::alert_config::{AlertConfig, NewAlertConfig};

/// Trait defining the alert config repository contract.
#[async_trait]
pub trait IAlertConfigRepository: Send + Sync {
    async fn all(&self) -> QueryResult<Vec<AlertConfig>>;
    async fn find(&self, id: &Uuid) -> QueryResult<AlertConfig>;
    async fn create(&self, item: &NewAlertConfig) -> QueryResult<AlertConfig>;
    async fn update(&self, id: &Uuid, item: &NewAlertConfig) -> QueryResult<AlertConfig>;
    async fn destroy(&self, id: &Uuid) -> QueryResult<usize>;
}