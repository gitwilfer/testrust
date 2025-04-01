use thiserror::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Errores de la capa de aplicación
/// 
/// Estos errores son independientes de la capa de presentación y representan
/// problemas de negocio, no de HTTP o presentación.
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Recurso no encontrado: {0}")]
    NotFound(String),
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Conflicto de datos: {0}")]
    Conflict(String),
    
    #[error("Error de autenticación: {0}")]
    AuthenticationError(String),
    
    #[error("Error de autorización: {0}")]
    AuthorizationError(String),
    
    #[error("Error de infraestructura: {0}")]
    InfrastructureError(String),
    
    #[error("Error inesperado: {0}")]
    UnexpectedError(String),
}

// Implementación para convertir anyhow::Error en ApplicationError
impl From<anyhow::Error> for ApplicationError {
    fn from(err: anyhow::Error) -> Self {
        ApplicationError::UnexpectedError(format!("{:?}", err))
    }
}

// No incluimos conversiones a tipos HTTP aquí para mantener la independencia