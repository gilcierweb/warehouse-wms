// pub mod auth_controller; // TODO: Re-enable when auth_controller is fixed
pub mod slots_controller;
pub mod movements_controller;
pub mod export_controller;
pub mod generic_controller;
pub mod health_controller;
pub mod users_controller;
pub mod auth_controller;
pub mod ws_controller;

use actix_web::{HttpResponse, Responder};
use diesel::QueryResult;
use serde::Serialize;
use rust_i18n::t;

pub fn handle_result<T: Serialize>(result: QueryResult<T>) -> impl Responder {
    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body(t!("crud.not_found")),
        Err(_) => HttpResponse::InternalServerError().body(t!("database.error")),
    }
}

pub fn handle_result_created<T: Serialize>(result: QueryResult<T>) -> impl Responder {
    match result {
        Ok(data) => HttpResponse::Created().json(data),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body(t!("crud.not_found")),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub fn handle_result_no_content(result: QueryResult<usize>) -> impl Responder {
    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body(t!("crud.not_found")),
        Err(_) => HttpResponse::InternalServerError().body(t!("database.error")),
    }
}