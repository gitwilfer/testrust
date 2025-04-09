use actix_web::web;
use crate::presentation::api::controllers::{user_controller, auth_controller};
use crate::presentation::api::middleware::{request_logger::RequestLoggerMiddleware, error_handler::ErrorHandlerMiddleware};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .wrap(RequestLoggerMiddleware)
        .wrap(ErrorHandlerMiddleware)
        //.configure(auth_controller::config)
        .configure(user_controller::config)
    );
}

/*
pub fn config(cfg: &mut web::ServiceConfig) {
    // Rutas públicas (sin autenticación)
    cfg.service(web::scope("/api")
        .service(web::scope("/auth")
            .configure(auth_controller::config)
        )
        .service(web::scope("/users")
            .service(user_controller::create_user) // Solo el endpoint de creación sin autenticación
        )
    );

    // Rutas protegidas (con autenticación)
    cfg.service(web::scope("/api")
        .wrap(AuthMiddleware::new(auth_service.clone()))
        .service(web::scope("/users")
            .service(user_controller::find_all_users)
            .service(user_controller::find_user_by_id)
            // ... otros endpoints de usuario
        )
    );
}

*/