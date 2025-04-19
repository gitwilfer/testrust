use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct AttributeDefinitionDto {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "data_type_id")] // Espera el NOMBRE del tipo de dato
    pub data_type_name: String,
    pub position: i16, // Directamente i16
    pub is_required: bool,
    pub is_unique: Option<i16>, // Directamente Option<i16>
    pub default_value: Option<String>,
    pub validation_regex: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateLogicalEntityRequest {
    pub name: String,
    pub description: Option<String>,
    pub assign_view: bool,
    // Añadir la lista de atributos
    pub attributes: Vec<AttributeDefinitionDto>,
}
