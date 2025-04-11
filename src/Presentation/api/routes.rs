use actix_web::web;
use crate::Presentation::api::controllers::{user_controller, auth_controller, health_controller};
use crate::Presentation::api::middleware::{request_logger::RequestLoggerMiddleware, error_handler::ErrorHandlerMiddleware};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .wrap(RequestLoggerMiddleware)
        .wrap(ErrorHandlerMiddleware)
        .configure(auth_controller::config)
        .configure(user_controller::config)
        .configure(health_controller::config)
    );
}
