use uuid::Uuid;
use serde::{Deserialize, Serialize}; // Si se necesita serializar

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)] // Añadir sqlx::FromRow si se usa query_as! directamente
pub struct DataType {
    pub id: Uuid,
    pub name: String,
    // pub description: Option<String>, // Opcional
}
