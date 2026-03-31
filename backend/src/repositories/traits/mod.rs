use diesel::QueryResult;
use async_trait::async_trait;

#[async_trait]
pub trait IRepository<M, N>: Send + Sync
where
    M: 'static,
    N: 'static,
{
    async fn all(&self) -> QueryResult<Vec<M>>;
    async fn find(&self, id: &uuid::Uuid) -> QueryResult<M>;
    async fn create(&self, item: &N) -> QueryResult<M>;
    async fn update(&self, id: &uuid::Uuid, item: &N) -> QueryResult<M>;
    async fn destroy(&self, id: &uuid::Uuid) -> QueryResult<usize>;
}
