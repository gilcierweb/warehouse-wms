#![allow(dead_code)]

use async_trait::async_trait;
use diesel::QueryResult;
use uuid::Uuid;

use crate::models::audit_log::{AuditLog, NewAuditLog};

/// Trait defining the audit logs repository contract.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IAuditLogRepository: Send + Sync {
    async fn all(&self) -> QueryResult<Vec<AuditLog>>;
    async fn find(&self, id: &Uuid) -> QueryResult<AuditLog>;
    async fn update(&self, id: &Uuid, item: &NewAuditLog) -> QueryResult<AuditLog>;
    async fn find_by_user(&self, user_id: &Uuid) -> QueryResult<Vec<AuditLog>>;
    async fn find_by_action(&self, action: &str) -> QueryResult<Vec<AuditLog>>;
    async fn create(&self, item: &NewAuditLog) -> QueryResult<AuditLog>;
    async fn destroy(&self, id: &Uuid) -> QueryResult<usize>;
}
