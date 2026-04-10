use crate::api_docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub use crate::controllers::{
    auth_controller, export_controller, health_controller, movements_controller, slots_controller,
    users_controller,
};

use crate::config::AppConfig;
use actix_web::web;
use std::sync::Arc;

pub fn config(
    cfg: &mut web::ServiceConfig,
    redis_pool: deadpool_redis::Pool,
    app_config: Arc<AppConfig>,
) {
    let openapi = ApiDoc::openapi();

    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
    )
    // WebSocket route - OUTSIDE /api/v1 scope (no API key middleware)
    // Authentication is done via JWT token in the first message
    .service(web::resource("/ws").route(web::get().to(crate::ws::server::ws_handler)))
    .service(
        web::scope("/api/v1")
            .wrap(crate::middleware::api_key_middleware::ApiKeyAuth::new(
                app_config.api_key.clone(),
            ))
            .wrap(crate::middleware::rate_limit_middleware::RateLimiter::new(
                redis_pool.clone(),
                crate::middleware::rate_limit_middleware::RATE_API,
            ))
            // Auth routes
            .service(
                web::scope("/auth")
                    .wrap(crate::middleware::rate_limit_middleware::RateLimiter::new(
                        redis_pool.clone(),
                        crate::middleware::rate_limit_middleware::RATE_AUTH,
                    ))
                    .service(auth_controller::login)
                    .service(auth_controller::register)
                    .service(auth_controller::confirm)
                    .service(auth_controller::me)
                    .service(auth_controller::refresh)
                    .service(auth_controller::logout)
                    .service(auth_controller::recover_password)
                    .service(auth_controller::reset_password)
                    .service(auth_controller::setup_2fa)
                    .service(auth_controller::enable_2fa)
                    .service(auth_controller::disable_2fa)
                    .service(auth_controller::change_password),
            )
            // User routes
            .service(users_controller::get_me)
            .service(users_controller::update_me)
            .service(users_controller::list_sessions)
            .service(users_controller::revoke_session)
            .service(users_controller::delete_me)
            .service(movements_controller::list_movements)
            .service(movements_controller::undo_movement)
            .service(movements_controller::create_movement)
            .service(movements_controller::get_movement_by_id)
            .service(movements_controller::get_movements)
            .service(movements_controller::update_movement_by_id)
            .service(movements_controller::delete_movement_by_id)
            .service(slots_controller::list_slots)
            .service(slots_controller::entry)
            .service(slots_controller::exit)
            .service(slots_controller::get_stats)
            .service(slots_controller::get_slot)
            .service(slots_controller::get_slot_by_id)
            .service(slots_controller::create_slot)
            .service(slots_controller::update_slot_by_id)
            .service(slots_controller::delete_slot_by_id)
            .service(export_controller::export_excel),
    );
}
