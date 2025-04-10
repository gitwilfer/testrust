use actix_web::web;
use crate::presentation::api::controllers::{user_controller, auth_controller};
use crate::presentation::api::middleware::{request_logger::RequestLoggerMiddleware, error_handler::ErrorHandlerMiddleware};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        //.wrap(RequestLoggerMiddleware)
        //.wrap(ErrorHandlerMiddleware)
        .configure(auth_controller::config)
        .configure(user_controller::config)
    );
}
