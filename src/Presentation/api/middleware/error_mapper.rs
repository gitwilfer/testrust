use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use anyhow::Error as AnyhowError;
use diesel::result::Error as DieselError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use log::error as log_error;

use crate::presentation::api::responses::{ApiError, ApiResponse};
use crate::presentation::api::validators::ValidationErrors;

// Un tipo de error simple que implementa Send y Sync
#[derive(Debug)]
pub struct AppError(pub String);

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(ApiError::internal_server_error(&self.0))
        )
    }
}

// Función original para mapear errores en controladores
pub fn map_error(err: AnyhowError) -> actix_web::Error {
    log_error!("Error: {:?}", err);
    
    // Manejar diferentes tipos de errores
    if let Some(diesel_err) = err.downcast_ref::<DieselError>() {
        match diesel_err {
            DieselError::NotFound => {
                return actix_web::error::ErrorNotFound(
                    ApiResponse::<()>::error(ApiError::not_found("Recurso no encontrado"))
                );
            }
            _ => {}
        }
    }
    
    if let Some(validation_errors) = err.downcast_ref::<ValidationErrors>() {
        return actix_web::error::ErrorBadRequest(
            ApiResponse::<()>::error(ApiError::bad_request(&validation_errors.to_string()))
        );
    }
    
    if let Some(api_error) = err.downcast_ref::<ApiError>() {
        return match api_error.status_code() {
            StatusCode::BAD_REQUEST => {
                actix_web::error::ErrorBadRequest(
                    ApiResponse::<()>::error(api_error.clone())
                )
            }
            StatusCode::NOT_FOUND => {
                actix_web::error::ErrorNotFound(
                    ApiResponse::<()>::error(api_error.clone())
                )
            }
            _ => {
                actix_web::error::ErrorInternalServerError(
                    ApiResponse::<()>::error(api_error.clone())
                )
            }
        };
    }
    
    // Error genérico
    actix_web::error::ErrorInternalServerError(
        ApiResponse::<()>::error(ApiError::internal_server_error(&format!("{:?}", err)))
    )
}

// Función thread-safe para el middleware
pub fn map_error_thread_safe(err: actix_web::Error) -> actix_web::Error {
    log_error!("Error en middleware: {:?}", err);
    
    // Convertir a un tipo de error thread-safe
    let message = format!("{:?}", err);
    AppError(message).into()
}
