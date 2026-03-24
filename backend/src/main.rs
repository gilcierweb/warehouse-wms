#[macro_use]
extern crate rust_i18n;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, Result, http::header, get, web};
use serde::Serialize;
use std::env;
use std::string::String;

use crate::config::AppConfig;

i18n!("locales");

// Import modules is required for use crate::mymod::
mod config;
mod errors;
mod db;
mod routes;
mod controllers;
mod models;
mod repositories;
mod auth;
mod ws;
mod middleware;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    
    let config = AppConfig::from_env();
    let config_data = web::Data::new(config.clone());
    
    let api_db = db::database::Database::new();
    let container = web::Data::new(
        repositories::AppContainer::new(api_db.pool.clone())
    );
    let app_data = web::Data::new(api_db);
    let ws_hub = web::Data::new(ws::server::WsHub::new());
        
    let port = config.port;
    let host = config.host.clone();
    let frontend_url: String = env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    println!("Running in http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&frontend_url)
                    .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .app_data(app_data.clone())
            .app_data(container.clone())
            .app_data(config_data.clone())
            .app_data(ws_hub.clone())
            .configure(routes::router::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
    })
    .bind((host, port))?
    .run()
    .await
}
