use serde::Deserialize;
use validator::Validate;
use serde::{de, Deserializer}; // Necesario para helpers
use std::str::FromStr; // Necesario para helpers

// Estructura para definir un atributo en el request
#[derive(Deserialize, Validate, Debug, Clone)]
pub struct AttributeDefinitionRequest {
    #[validate(length(min = 1, max = 100, message = "Attribute name must be between 1 and 100 characters"))]
    pub name: String,
    pub description: Option<String>,

    #[validate(length(min = 1, message = "Attribute data_type_id (name) cannot be empty"))]
    pub data_type_id: String, // Considera si debería ser Uuid o i32 si es un ID

    #[validate(range(min = 0, max = 100, message = "Position must be between 0 and 100"))]
    #[serde(default, deserialize_with = "deserialize_string_to_i16")]
    pub position: i16,

    #[serde(default, deserialize_with = "deserialize_string_to_bool")]
    pub is_required: bool,

    #[serde(default, deserialize_with = "deserialize_optional_string_to_i16")]
    #[validate(custom(function = "validate_is_unique_range", message = "is_unique must be between 0 and 10"))]
    pub is_unique: Option<i16>,

    pub default_value: Option<String>,
    pub validation_regex: Option<String>,
}

// Estructura principal del request (sin cambios aquí)
#[derive(Deserialize, Validate, Debug, Clone)]pub struct CreateEntityWithAttributesRequest {
    #[serde(rename = "Entity")]
    #[validate(length(min = 1, message = "Entity name cannot be empty"))]
    pub entity_name: String,

    #[serde(rename = "Attribute")]
    #[validate]
    #[validate(length(min = 1, message = "At least one attribute must be provided"))]
    pub attributes: Vec<AttributeDefinitionRequest>,
}

// --- Funciones Helper para Deserialización Flexible (sin cambios aquí) ---
// fn deserialize_string_to_bool<'de, D>(...) -> Result<bool, D::Error> { ... }
// fn deserialize_string_to_i16<'de, D>(...) -> Result<i16, D::Error> { ... }
// fn deserialize_optional_string_to_i16<'de, D>(...) -> Result<Option<i16>, D::Error> { ... }

// --- Función de Validación Custom para is_unique ---
fn validate_is_unique_range(value: &Option<i16>) -> Result<(), validator::ValidationError> {
    if let Some(val) = value {
        if *val < 0 || *val > 10 {
            return Err(validator::ValidationError::new("is_unique_range_invalid"));
        }
    }
    Ok(())
}

fn deserialize_string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => Err(de::Error::custom(format!("invalid boolean string: {}", s))),
    }
}

/// Deserializa un string a i16.
fn deserialize_string_to_i16<'de, D>(deserializer: D) -> Result<i16, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    i16::from_str(&s).map_err(|_| de::Error::custom(format!("invalid i16 string: {}", s)))
}

fn deserialize_optional_string_to_i16<'de, D>(deserializer: D) -> Result<Option<i16>, D::Error>
where
    D: Deserializer<'de>,
{
    // Intenta deserializar como Option<String> primero para manejar null/ausente
    let opt_s = Option::<String>::deserialize(deserializer)?;
    match opt_s {
        Some(s) if !s.is_empty() => {
            // Si hay un string no vacío, intenta parsearlo
            i16::from_str(&s)
                .map(Some) // Envuelve el resultado en Some
                .map_err(|_| de::Error::custom(format!("invalid optional i16 string: {}", s)))
        }
        _ => Ok(None), // Si es None, null o string vacío, resulta en None
    }
}


