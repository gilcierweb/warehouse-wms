use diesel::QueryResult;
use uuid::Uuid;
use async_trait::async_trait;

use crate::models::alert_config::{AlertConfig, NewAlertConfig};
use crate::repositories::traits::IRepository;

/// Trait defining the alert config repository contract.
#[async_trait]
pub trait IAlertConfigRepository: IRepository<AlertConfig, NewAlertConfig> + Send + Sync {}