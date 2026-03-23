pub use crate::controllers::{movements_controller, slots_controller, export_controller};
use actix_web::web;
use crate::ws::server::ws_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
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
