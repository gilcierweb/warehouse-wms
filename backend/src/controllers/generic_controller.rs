use actix_web::{web, Responder};
use std::sync::Arc;
use uuid::Uuid;

use crate::repositories::traits::IRepository;

/// Generic get_all that works with any repository implementing IRepository
pub async fn get_all<M, N, R>(repo: Arc<R>) -> impl Responder
where
    M: serde::Serialize + 'static,
    N: 'static,
    R: IRepository<M, N> + ?Sized,
{
    super::handle_result(repo.all().await)
}

/// Generic get_by_id that works with any repository implementing IRepository
pub async fn get_by_id<M, N, R>(repo: Arc<R>, id: web::Path<Uuid>) -> impl Responder
where
    M: serde::Serialize + 'static,
    N: 'static,
    R: IRepository<M, N> + ?Sized,
{
    super::handle_result(repo.find(&id).await)
}

/// Generic create that works with any repository implementing IRepository
pub async fn create<M, N, R>(repo: Arc<R>, item: web::Json<N>) -> impl Responder
where
    M: serde::Serialize + 'static,
    N: Clone + 'static,
    R: IRepository<M, N> + ?Sized,
{
    super::handle_result_created(repo.create(&item).await)
}

/// Generic update that works with any repository implementing IRepository
pub async fn update<M, N, R>(repo: Arc<R>, id: web::Path<Uuid>, item: web::Json<N>) -> impl Responder
where
    M: serde::Serialize + 'static,
    N: Clone + 'static,
    R: IRepository<M, N> + ?Sized,
{
    super::handle_result(repo.update(&id, &item).await)
}

/// Generic delete that works with any repository implementing IRepository
pub async fn delete<M, N, R>(repo: Arc<R>, id: web::Path<Uuid>) -> impl Responder
where
    M: 'static,
    N: 'static,
    R: IRepository<M, N> + ?Sized,
{
    super::handle_result_no_content(repo.destroy(&id).await)
}
