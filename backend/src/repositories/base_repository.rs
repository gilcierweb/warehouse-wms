use uuid::Uuid;
use async_trait::async_trait;

use crate::db::database::Database;
use actix_web::web;

/// Legacy base repository trait — kept for backward compatibility.
/// New code should use the specific I*Repository traits from each module.
#[async_trait]
pub trait BaseRepository<TEntity> {
    /// create a new repository with the connection
    fn new(connection: web::Data<Database>) -> Self;
    async fn all(&self) -> Result<Vec<TEntity>, diesel::result::Error>;
    async fn find(&self, id: &Uuid) -> Option<TEntity>;
    async fn create(&mut self, entity: &mut TEntity) -> Result<TEntity, std::fmt::Error>;
    async fn update(&mut self, id: &Uuid, entity: &mut TEntity) -> Option<TEntity>;
    async fn delete(&mut self, id: &Uuid) -> Option<usize>;
}