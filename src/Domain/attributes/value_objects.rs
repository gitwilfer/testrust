use crate::Domain::error::DomainError;
use serde::{Deserialize, Serialize}; // Para DTOs

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)] // Hashable para validación de duplicados
pub struct AttributeName(String);

impl AttributeName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        if name.is_empty() || name.len() > 100 { // Validar longitud según schema
            Err(DomainError::Validation("Attribute name is invalid".to_string()))
        } else {
            // Podría añadir validación de caracteres si es necesario
            Ok(Self(name))
        }
    }
    pub fn into_inner(self) -> String { self.0 }
    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Position(i16); // SMALLINT

impl Position {
    pub fn new(pos: i16) -> Result<Self, DomainError> {
        // Según desarrollo_beta.txt: 0-99 (aunque schema permite más)
        // Ajustar si la regla es diferente
        if pos < 0 || pos > 99 {
             Err(DomainError::Validation(format!("Position {} is out of range (0-99)", pos)))
        } else {
            Ok(Self(pos))
        }
    }
     pub fn value(&self) -> i16 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniquenessGroup(i16); // SMALLINT

impl UniquenessGroup {
     pub fn new(group: i16) -> Result<Self, DomainError> {
        // Según desarrollo_beta.txt: 1-10 (y no negativo)
        // Schema permite NULL o cualquier SMALLINT. Clarificar regla exacta.
        // Implementando la regla de beta.txt por ahora:
        if group < 1 || group > 10 {
             Err(DomainError::Validation(format!("Uniqueness group {} is out of range (1-10)", group)))
        } else {
            Ok(Self(group))
        }
    }
     pub fn value(&self) -> i16 { self.0 }
}

// Otros VOs si son necesarios...
