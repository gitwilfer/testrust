use actix_web::web;
use crate::Presentation::api::controllers::{user_controller, auth_controller, health_controller, logical_entity_controller};
use crate::Presentation::api::middleware::{request_logger::RequestLoggerMiddleware, error_handler::ErrorHandlerMiddleware, auth_middleware::AuthMiddleware};

/// Configura las rutas de la API con middleware aplicado selectivamente.
/// La ruta de login no requiere token, las demás sí.
pub fn config(cfg: &mut web::ServiceConfig) {
    // Crear instancia de AuthMiddleware
    let auth_middleware = AuthMiddleware::new();

    // Rutas protegidas con middleware de autenticación
    cfg.service(
        web::scope("/api/users")
            .wrap(RequestLoggerMiddleware)
            .wrap(ErrorHandlerMiddleware)
            //.wrap(auth_middleware.clone()) //PENDIENTE DE SOLUCIONAR ERROR, DE block_in_place EN aut_middleware
            .configure(user_controller::config)
    );

    cfg.service(
        web::scope("/api/logical-entities") // Define el prefijo base
            .wrap(RequestLoggerMiddleware)
            .wrap(ErrorHandlerMiddleware)
            // Probablemente quieras proteger estas rutas también:
            //.wrap(auth_middleware.clone()) // PENDIENTE
            .configure(logical_entity_controller::config) // Delega al config del nuevo controlador
    );

    cfg.service(
        web::scope("/api/health")
            .wrap(RequestLoggerMiddleware)
            .wrap(ErrorHandlerMiddleware)
            //.wrap(auth_middleware)
            .configure(health_controller::config)
    );

    // Rutas sin autenticación (login)
    cfg.service(
        web::scope("/api")
            .wrap(RequestLoggerMiddleware)
            .wrap(ErrorHandlerMiddleware)
            .configure(auth_controller::config)
    );
}