use std::sync::Arc;
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use rust_i18n::t;

use crate::repositories::traits::IRepository;

pub async fn get_all<M, N, R>(repo: &Arc<R>) -> HttpResponse
where
    M: serde::Serialize + 'static,
    N: 'static,
    R: IRepository<M, N> + ?Sized,
{
    match repo.all().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body(t!("crud.not_found")),
        Err(_) => HttpResponse::InternalServerError().body(t!("database.error")),
    }
}

pub async fn get_by_id<M, N, R>(repo: &Arc<R>, id: web::Path<Uuid>) -> HttpResponse
where
    M: serde::Serialize + 'static,
    N: 'static,
    R: IRepository<M, N> + ?Sized,
{
    match repo.find(&id).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body(t!("crud.not_found")),
        Err(_) => HttpResponse::InternalServerError().body(t!("database.error")),
    }
}

pub async fn create<M, N, R>(repo: &Arc<R>, item: web::Json<N>) -> HttpResponse
where
    M: serde::Serialize + 'static,
    N: Clone + 'static,
    R: IRepository<M, N> + ?Sized,
{
    match repo.create(&item).await {
        Ok(data) => HttpResponse::Created().json(data),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body(t!("crud.not_found")),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn update<M, N, R>(repo: &Arc<R>, id: web::Path<Uuid>, item: web::Json<N>) -> HttpResponse
where
    M: serde::Serialize + 'static,
    N: Clone + 'static,
    R: IRepository<M, N> + ?Sized,
{
    match repo.update(&id, &item).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body(t!("crud.not_found")),
        Err(_) => HttpResponse::InternalServerError().body(t!("database.error")),
    }
}

pub async fn delete<M, N, R>(repo: &Arc<R>, id: web::Path<Uuid>) -> HttpResponse
where
    M: serde::Serialize + 'static,
    N: 'static,
    R: IRepository<M, N> + ?Sized,
{
    match repo.destroy(&id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body(t!("crud.not_found")),
        Err(_) => HttpResponse::InternalServerError().body(t!("database.error")),
    }
}
