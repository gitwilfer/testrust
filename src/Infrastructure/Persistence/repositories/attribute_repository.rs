// src/Infrastructure/persistence/repositories/attribute_repository.rs (Ejemplo Síncrono con Diesel)
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::schema::attributes; // Asegúrate que el schema de Diesel esté definido
use crate::Domain::attributes::{
    attribute::Attribute,
    repository::AttributeRepository,
};
use crate::Domain::error::DomainError;
use uuid::Uuid;
use std::sync::Arc; // Si se usa Arc para el pool

// Struct para insertar con Diesel
#[derive(Insertable)]
#[diesel(table_name = attributes)]
struct NewAttribute<'a> {
    id: Uuid,
    entity_id: Uuid,
    data_type_id: Uuid,
    name: &'a str,
    description: Option<&'a str>,
    is_required: bool,
    position: i16,
    is_unique: Option<i16>,
    default_value: Option<&'a str>,
    validation_regex: Option<&'a str>,
    created_by: Uuid,
    // created_at se pone por defecto en DB
    status: i16,
}

// Implementación del repositorio
pub struct DieselAttributeRepository {
    // Podría necesitar el pool si la conexión no se pasa directamente
     pool: Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
}

impl DieselAttributeRepository {
    pub fn new(pool: Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>) -> Self {
        Self { pool }
    }
}

// Implementación SÍNCRONA del trait
// NOTA: La gestión de la transacción (obtener conn, begin, commit/rollback)
// se hará en la capa de Aplicación. Este método asume que se ejecuta
// dentro de una transacción existente.
impl AttributeRepository for DieselAttributeRepository {
     fn save_batch_sync(
        &self,
        // conn: &mut PgConnection, // Recibir la conexión transaccional
        entity_id: Uuid,
        user_id: Uuid,
        attributes_domain: &[Attribute],
    ) -> Result<(), DomainError> {

        let mut conn = self.pool.get().map_err(|e| DomainError::DatabaseError(format!("Failed to get connection: {}", e)))?;

        let new_attributes_to_insert: Vec<NewAttribute> = attributes_domain
            .iter()
            .map(|attr| NewAttribute {
                id: attr.id, // Usar el ID pre-generado
                entity_id, // Usar el ID de la entidad padre
                data_type_id: attr.data_type_id,
                name: attr.name.as_str(),
                description: attr.description.as_deref(),
                is_required: attr.is_required,
                position: attr.position.value(),
                is_unique: attr.is_unique.map(|u| u.value()),
                default_value: attr.default_value.as_deref(),
                validation_regex: attr.validation_regex.as_deref(),
                created_by: user_id,
                status: attr.status,
            })
            .collect();

        // Ejecutar la inserción dentro de la transacción proporcionada
        diesel::insert_into(attributes::table)
            .values(&new_attributes_to_insert)
            .execute(&mut conn) // Usar la conexión transaccional
            .map_err(|db_err| {
                eprintln!("Failed to insert attributes: {:?}", db_err);
                // Mapear errores específicos si es necesario (ej: UNIQUE constraint)
                DomainError::DatabaseError(format!("Failed to save attributes: {}", db_err))
            })?;

        Ok(())
    }
}
