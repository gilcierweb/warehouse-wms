#[macro_use]
extern crate rust_i18n;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware::Logger, web};
use deadpool_redis::{Config as RedisConfig, Runtime};
use serde::Serialize;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api_docs;
mod auth;
mod config;
mod controllers;
mod db;
mod errors;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;
mod ws;

use config::AppConfig;
use db::database::{DBPool, Database};
use ws::server::{WsHub, HubData};

i18n!("locales");

fn set_default_locale() {
    rust_i18n::set_locale("pt-BR");
}

#[derive(Clone)]
pub struct AppState {
    pub db: DBPool,
    pub redis: deadpool_redis::Pool,
    pub config: Arc<AppConfig>,
}

#[derive(Serialize)]
pub struct Response<'a> {
    pub message: Cow<'a, str>,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: t!("health.ok"),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse, actix_web::Error> {
    let response = Response {
        message: t!("errors.not_found", resource = "Resource"),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set default locale to pt-BR
    set_default_locale();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "justfans_api=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    dotenvy::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load configuration");
    let config = Arc::new(config);
    tracing::info!(
        "Starting JustFans API v{} on {}:{}",
        env!("CARGO_PKG_VERSION"),
        config.host,
        config.port
    );

    let api_db = Database::new();
    let db_pool = api_db.pool.clone();
    let db_pool_for_container = db_pool.clone();

    let redis_cfg = RedisConfig::from_url(&config.redis_url);

    let redis_pool = redis_cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis connection pool");
    let redis_pool_for_container = redis_pool.clone();

    let state = web::Data::new(AppState {
        db: db_pool,
        redis: redis_pool.clone(),
        config: config.clone(),
    });

    let container = web::Data::new(repositories::AppContainer::new(
        db_pool_for_container,
        config.clone(),
    ));

    let hub = web::Data::new(WsHub::new());
    // TODO: Re-enable when auction_scheduler is implemented
    // services::auction_scheduler::spawn_auction_close_scheduler(container.clone());

    let cors_origins = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:3000,http://localhost:3001,http://127.0.0.1:3000,http://127.0.0.1:3001".to_string());

    let host = config.host.clone();
    let port = config.port;

    println!("Running in http://localhost:{}", port);

    HttpServer::new(move || {
        let pool_for_router = state.redis.clone();

        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::ACCESS_CONTROL_REQUEST_HEADERS,
                actix_web::http::header::HeaderName::from_static("x-api-key"),
            ])
            .supports_credentials()
            .max_age(3600);

        for origin in cors_origins.split(',') {
            let origin = origin.trim();
            if !origin.is_empty() {
                cors = cors.allowed_origin(origin);
            }
        }

        App::new()
            .app_data(state.clone())
            .app_data(container.clone())
            .app_data(hub.clone())
            .app_data(web::JsonConfig::default().limit(1024 * 1024))
            .wrap(cors)
            .wrap(Logger::new("%a \"%r\" %s %b %Dms"))
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(actix_web::middleware::Compress::default())
            .configure(|cfg| routes::router::config(cfg, redis_pool.clone(), config.clone()))
            .default_service(web::route().to(not_found))
    })
        .bind((host, port))?
        .run()
        .await
}