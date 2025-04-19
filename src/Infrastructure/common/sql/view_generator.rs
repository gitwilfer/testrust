use crate::Domain::attributes::attribute::Attribute; // Asume que tienes Attribute
use crate::Domain::data_types::data_type::DataType; // Asume que tienes DataType
use crate::Domain::logical_entities::logical_entity::LogicalEntity; // Asume que tienes LogicalEntity
use crate::Domain::error::DomainError;
use sqlx::postgres::PgRow; // Para mapeo si es necesario
use sqlx::Row;
use uuid::Uuid;

// Estructura auxiliar para pasar datos necesarios
pub struct AttributeInfo<'a> {
   pub attribute: &'a Attribute,
   pub data_type_name: &'a str, // Necesitamos el nombre del tipo ('string', 'integer', etc.)
}

pub fn generate_view_sql(
    entity: &LogicalEntity,
    attributes_info: &[AttributeInfo<'_>], // Atributos con su tipo de dato asociado
) -> Result<String, DomainError> {
    if attributes_info.is_empty() {
        // Opcional: decidir si crear una vista sin columnas EAV o devolver error
        tracing::warn!("Attempted to generate view for entity {} with no attributes.", entity.id);
         return Err(DomainError::Validation("Cannot generate view with no attributes".to_string()));
    }

    let view_name = format!("view_{}", entity.name.as_str()); // Asume LogicalEntity tiene un name: LogicalEntityName

    // --- Construcción de la parte SELECT ---
    let mut select_clauses: Vec<String> = vec![
        "t.id".to_string(), // ID de la tupla/instancia
        // Seleccionar usernames si es posible
        "COALESCE(creator.username, t.created_by::text) AS created_by".to_string(),
        "t.created_at".to_string(),
        "COALESCE(updater.username, t.updated_by::text) AS updated_by".to_string(),
        "t.updated_at".to_string(),
        "t.status".to_string(), // Status de la tupla
    ];

    // --- Construcción de la parte FROM y JOINs ---
    let mut join_clauses: Vec<String> = vec![
        "FROM tuplas t".to_string(),
        "LEFT JOIN users creator ON t.created_by = creator.id".to_string(),
        "LEFT JOIN users updater ON t.updated_by = updater.id".to_string(),
    ];

    // Ordenar atributos por posición para el orden de las columnas en la vista
    let mut sorted_attributes = attributes_info.to_vec(); // Clonar para ordenar
    sorted_attributes.sort_by_key(|a| a.attribute.position.value()); // Asume position tiene value() -> i16

    for (index, attr_info) in sorted_attributes.iter().enumerate() {
        let attribute = attr_info.attribute;
        let data_type_name = attr_info.data_type_name;
        let attribute_name = attribute.name.as_str(); // Asume name tiene as_str()
        let attribute_id = attribute.id; // Asume attribute tiene id: Uuid
        let alias = format!("av_{}", index); // Alias único para cada join a attribute_values

        // Mapear nombre de tipo de dato a columna de valor en attribute_values
        let value_column = match data_type_name {
            "string" => "string_value",
            "text" => "text_value",
            "integer" => "integer_value",
            "float" => "float_value",
            "numeric" => "numeric_value",
            "boolean" => "boolean_value",
            "datetime" => "datetime_value",
            "date" => "date_value",
            "time" => "time_value",
            "uuid" => "uuid_value",
            "json" => "json_value", // Asumiendo JSONB se maneja como JSON
            "binary" => "binary_value",
            // Añadir más tipos según data_types
            _ => return Err(DomainError::Validation(format!("Unsupported data type '{}' for view generation", data_type_name))),
        };

        // Añadir JOIN para este atributo
        join_clauses.push(format!(
            "LEFT JOIN attribute_values {} ON t.id = {}.instance_id AND {}.attribute_id = '{}'",
            alias, alias, alias, attribute_id
        ));

        // Añadir SELECT para este atributo, usando comillas dobles para el alias
        select_clauses.push(format!(
            "{}.{} AS \"{}\"", // Usar comillas dobles para el alias
            alias, value_column, attribute_name
        ));
    }

    // --- Ensamblar la consulta completa ---
    let select_sql = select_clauses.join(",\n    ");
    let join_sql = join_clauses.join("\n  ");
    let where_sql = format!("WHERE t.entity_id = '{}'", entity.id); // Filtrar por entity_id

    // Usar CREATE OR REPLACE VIEW para idempotencia
    let final_sql = format!(
        "CREATE OR REPLACE VIEW \"{}\" AS\nSELECT\n    {}\n  {}\n  {};",
        view_name, // Usar comillas dobles por si el nombre tiene mayúsculas/símbolos
        select_sql,
        join_sql,
        where_sql
    );

    Ok(final_sql)
}
