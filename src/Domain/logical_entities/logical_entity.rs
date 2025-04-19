// src/Domain/logical_entities/logical_entity.rs

use chrono::{DateTime, Utc};
use uuid::Uuid;

// Representa una entidad lógica definida por el usuario (mapeada desde la tabla 'entities').
#[derive(Debug, Clone)] // Añadir PartialEq, Eq si es necesario para comparaciones
pub struct LogicalEntity {
    pub id: Uuid,
    pub name: String, // Podría ser un Value Object como LogicalEntityName
    pub description: Option<String>,
    pub assign_view: bool,
    pub created_by: Uuid, // Podría ser un Value Object UserId
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>, // Podría ser un Value Object UserId
    pub updated_at: Option<DateTime<Utc>>,
    pub status: i16, // Considerar un Enum para el status si hay varios estados definidos
}

// Aquí se podrían añadir métodos de dominio para LogicalEntity si fuera necesario.
// Ejemplo:
// impl LogicalEntity {
//     pub fn new(...) -> Result<Self, DomainError> { ... }
//     pub fn update_description(&mut self, description: Option<String>, updated_by: Uuid) { ... }
//     pub fn deactivate(&mut self, updated_by: Uuid) { ... }
// }

// Considerar definir un DomainError específico para este módulo.