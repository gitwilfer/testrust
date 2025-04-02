// src/domain/errors.rs
use thiserror::Error;
use std::fmt;

/// Errores específicos del dominio
/// 
/// Estos errores representan violaciones de reglas del negocio
/// y son independientes de la infraestructura o presentación.
#[derive(Debug, Error, Clone)]
pub enum DomainError {
    #[error("Violación de regla de validación: {0}")]
    ValidationError(String),
    
    #[error("Entidad no encontrada: {0}")]
    NotFoundError(String),
    
    #[error("Operación no permitida: {0}")]
    ForbiddenOperation(String),
    
    #[error("Estado inválido: {0}")]
    InvalidState(String),
    
    #[error("Error de dominio: {0}")]
    GenericDomainError(String),
}

// Implementación que permite convertir anyhow::Error en DomainError
impl From<anyhow::Error> for DomainError {
    fn from(err: anyhow::Error) -> Self {
        DomainError::GenericDomainError(format!("{:?}", err))
    }
}

/// Resultado específico del dominio
/// 
/// Este tipo simplifica el manejo de errores en la capa de dominio.
pub type DomainResult<T> = Result<T, DomainError>;

/// Extensión para convertir errores de dominio a ApplicationError
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_domain_error_creation() {
        let error = DomainError::ValidationError("El campo es inválido".to_string());
        assert!(error.to_string().contains("Violación de regla de validación"));
        
        let error = DomainError::NotFoundError("Usuario".to_string());
        assert!(error.to_string().contains("Entidad no encontrada"));
    }
    
    #[test]
    fn test_convert_anyhow_to_domain_error() {
        let anyhow_error = anyhow::anyhow!("Error de prueba");
        let domain_error: DomainError = anyhow_error.into();
        
        match domain_error {
            DomainError::GenericDomainError(msg) => {
                assert!(msg.contains("Error de prueba"));
            },
            _ => panic!("Conversión incorrecta de anyhow::Error"),
        }
    }
}