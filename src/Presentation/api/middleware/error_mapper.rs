use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use anyhow::Error as AnyhowError;
use diesel::result::Error as DieselError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use log::error as log_error;
use crate::application::errors::application_error::ApplicationError;
use crate::presentation::api::responses::{ApiResponse, ApiError};
use crate::presentation::api::validators::ValidationErrors;

// Un tipo de error simple que implementa Send y Sync
#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub status_code: StatusCode,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }
    
    fn error_response(&self) -> HttpResponse {
        let api_error = ApiError::new(self.status_code, &self.message);
        HttpResponse::build(self.status_code)
            .json(ApiResponse::<()>::error(api_error))
    }
}

// Función para mapear ApplicationError a actix_web::Error
pub fn map_error(err: AnyhowError) -> actix_web::Error {
    log_error!("Error: {:?}", err);
    
    // Extraer ApplicationError si está presente
    if let Some(app_error) = err.downcast_ref::<ApplicationError>() {
        let (status_code, message) = match app_error {
            ApplicationError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApplicationError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApplicationError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            ApplicationError::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            ApplicationError::AuthorizationError(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            ApplicationError::InfrastructureError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ApplicationError::UnexpectedError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };
        
        return AppError {
            message,
            status_code,
        }.into();
    }
    
    // Manejar DieselError específicamente
    if let Some(diesel_err) = err.downcast_ref::<DieselError>() {
        match diesel_err {
            DieselError::NotFound => {
                return AppError {
                    message: "Recurso no encontrado".to_string(),
                    status_code: StatusCode::NOT_FOUND,
                }.into();
            }
            _ => {}
        }
    }
    
    // Manejar errores de validación
    if let Some(validation_errors) = err.downcast_ref::<ValidationErrors>() {
        return AppError {
            message: validation_errors.to_string(),
            status_code: StatusCode::BAD_REQUEST,
        }.into();
    }
    
    // Error genérico
    AppError {
        message: format!("Error interno del servidor: {:?}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    }.into()
}

// Función thread-safe para el middleware
pub fn map_error_thread_safe(err: actix_web::Error) -> actix_web::Error {
    log_error!("Error en middleware: {:?}", err);
    
    // Convertir a un tipo de error thread-safe
    let message = format!("{:?}", err);
    AppError {
        message,
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    }.into()
}