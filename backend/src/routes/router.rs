pub use crate::controllers::{
    auth_controller, export_controller, movements_controller, slots_controller,
};
use crate::ws::server::ws_handler;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .service(auth_controller::login)
                    .service(auth_controller::register)
                    .service(auth_controller::me)
                    .service(auth_controller::recover_password)
                    .service(auth_controller::reset_password),
            )
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
            .service(export_controller::export_excel),
    )
    .route("/ws/live", web::get().to(ws_handler));
}
