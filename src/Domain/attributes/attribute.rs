use uuid::Uuid;
use super::value_objects::*; // Importar VOs definidos abajo

#[derive(Debug, Clone)] // No necesita ser serializable aquí
pub struct Attribute {
    pub id: Uuid, // Generado antes de guardar o por la DB
    pub entity_id: Uuid, // FK a LogicalEntity (entities.id)
    pub data_type_id: Uuid, // FK a data_types.id
    pub name: AttributeName,
    pub description: Option<String>,
    pub is_required: bool,
    pub position: Position,
    pub is_unique: Option<UniquenessGroup>, // Usar SMALLINT (i16)
    pub default_value: Option<String>,
    pub validation_regex: Option<String>,
    pub status: i16, // Default a 1 (Active)
    // Audit fields (created_by, etc.) se manejarán al guardar
}

impl Attribute {
    // Constructor o método 'new' para crear instancias,
    // aplicando validaciones iniciales si es posible con VOs.
    pub fn new(
        entity_id: Uuid,
        data_type_id: Uuid,
        name: AttributeName,
        description: Option<String>,
        is_required: bool,
        position: Position,
        is_unique: Option<UniquenessGroup>,
        default_value: Option<String>,
        validation_regex: Option<String>,
        // status se puede poner por defecto
    ) -> Self {
        Self {
            id: Uuid::new_v4(), // Generar ID aquí o dejar que la DB lo haga
            entity_id,
            data_type_id,
            name,
            description,
            is_required,
            position,
            is_unique,
            default_value,
            validation_regex,
            status: 1, // Default a activo
        }
    }
}
