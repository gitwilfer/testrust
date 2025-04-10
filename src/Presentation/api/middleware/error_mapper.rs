use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use anyhow::Error as AnyhowError;
//use diesel::result::Error as DieselError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use log::error as log_error;
use crate::Application::errors::application_error::ApplicationError;
use crate::Presentation::api::adapters::ErrorAdapter;
//use crate::Presentation::api::validators::ValidationErrors;

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
        HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": {
                "code": 500,
                "message": self.0
            }
        }))
    }
}

// Función para mapear errores en controladores
pub fn map_error(err: AnyhowError) -> actix_web::Error {
    log_error!("Error: {:?}", err);
    
    // Si es un ApplicationError, delegamos al adaptador específico
    if let Some(app_error) = err.downcast_ref::<ApplicationError>() {
        return actix_web::error::InternalError::from_response(
            format!("{:?}", app_error),
            ErrorAdapter::map_application_error(app_error.clone())
        ).into();
    }
    
    // Para otros tipos de errores, usamos nuestro AppError
    AppError(format!("Error interno: {:?}", err)).into()
}

// Función thread-safe para el middleware
pub fn map_error_thread_safe(err: actix_web::Error) -> actix_web::Error {
    log_error!("Error en middleware: {:?}", err);
    
    // Convertir a un tipo de error thread-safe
    let message = format!("{:?}", err);
    AppError(message).into()
}