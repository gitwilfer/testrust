use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use log::error;
use crate::Application::errors::application_error::ApplicationError;
use crate::Presentation::api::responses::{ApiResponse, ApiError};

/// Adaptador para convertir errores de aplicación a respuestas HTTP
/// 
/// Este adaptador es responsable de mapear los errores de dominio y aplicación
/// a respuestas HTTP apropiadas, manteniendo la separación de capas.
pub struct ErrorAdapter;

impl ErrorAdapter {
    /// Mapea un error de aplicación a una respuesta HTTP
    /// 
    /// Esta función realiza el mapeo entre los errores de la capa de aplicación
    /// y las respuestas HTTP correspondientes, manteniendo la semántica del error.
    pub fn map_application_error(err: ApplicationError) -> HttpResponse {
        // Registrar el error para depuración
        error!("Error de aplicación: {:?}", err);
        
        match err {
            ApplicationError::NotFound(msg) => {
                HttpResponse::NotFound().json(ApiResponse::<()>::error(ApiError::not_found(&msg)))
            },
            ApplicationError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(ApiResponse::<()>::error(ApiError::bad_request(&msg)))
            },
            ApplicationError::Conflict(msg) => {
                HttpResponse::Conflict().json(ApiResponse::<()>::error(ApiError::new(StatusCode::CONFLICT, &msg)))
            },
            ApplicationError::AuthenticationError(msg) => {
                HttpResponse::Unauthorized().json(ApiResponse::<()>::error(ApiError::new(StatusCode::UNAUTHORIZED, &msg)))
            },
            ApplicationError::AuthorizationError(msg) => {
                HttpResponse::Forbidden().json(ApiResponse::<()>::error(ApiError::new(StatusCode::FORBIDDEN, &msg)))
            },
            ApplicationError::InfrastructureError(msg) => {
                error!("Error de infraestructura: {}", msg);
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(ApiError::internal_server_error(&msg)))
            },
            ApplicationError::UnexpectedError(msg) => {
                error!("Error inesperado: {}", msg);
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(ApiError::internal_server_error(&msg)))
            },
        }
    }
    
    /// Envuelve cualquier error anyhow como respuesta HTTP
    pub fn map_anyhow_error(err: anyhow::Error) -> HttpResponse {
        error!("Error general: {:?}", err);
        
        // Intentar convertir a ApplicationError si es posible
        if let Some(app_error) = err.downcast_ref::<ApplicationError>() {
            return Self::map_application_error(app_error.clone());
        }
        
        // Si no es un ApplicationError, devolverlo como error interno
        HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            ApiError::internal_server_error(&format!("Error interno: {}", err))
        ))
    }
}