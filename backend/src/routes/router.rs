pub use crate::controllers::movements_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(movements_controller::list_movements)
            .service(movements_controller::undo_movement)
            .service(movements_controller::create_movement)
            .service(movements_controller::get_movement_by_id)
            .service(movements_controller::get_movements)
            .service(movements_controller::update_movement_by_id)
            .service(movements_controller::delete_movement_by_id),
    );
}
