use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use crate::application::errors::application_error::ApplicationError;
use crate::presentation::api::models::response::{ApiResponse, ApiError};

/// Adaptador para convertir errores de aplicación a respuestas HTTP
/// 
/// Este adaptador es responsable de mapear los errores de dominio y aplicación
/// a respuestas HTTP apropiadas, manteniendo la separación de capas.
pub struct ErrorAdapter;

impl ErrorAdapter {
    pub fn map_application_error(err: ApplicationError) -> HttpResponse {
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
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(ApiError::internal_server_error(&msg)))
            },
            ApplicationError::UnexpectedError(msg) => {
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(ApiError::internal_server_error(&msg)))
            },
        }
    }
}
