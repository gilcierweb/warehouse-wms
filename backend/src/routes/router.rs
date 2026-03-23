use actix_web::web;


pub fn config(cfg: &mut web::ServiceConfig) {
      cfg.service(
           web::scope("/api")
               // .service(todos_controller::create_todo)
      )
}