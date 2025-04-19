// src/Infrastructure/repositories/attribute_command_repository_impl.rs

use async_trait::async_trait;
use diesel::prelude::*; // Necesario para Insertable, etc.
use diesel_async::{AsyncPgConnection, RunQueryDsl}; // Conexión y métodos async
use std::error::Error;
use uuid::Uuid;
use anyhow::Context; // Para contexto de errores
use log::debug; // Para logging

// Importar el trait del Port
use crate::Application::ports::driven::repositories::AttributeCommandRepository;
// Importar el schema de la tabla attributes
use crate::Infrastructure::Persistence::schema::attributes;
// Importar (o definir) el modelo Diesel para attributes si es necesario para Insertable
// use crate::Infrastructure::Persistence::models::AttributeModel;

// Implementación del repositorio de comandos para Atributos
// Puede ser ZST si no tiene estado propio
#[derive(Clone, Copy)]
pub struct AttributeCommandRepositoryImpl;

impl AttributeCommandRepositoryImpl {
    pub fn new() -> Self {
        Self // Constructor simple para ZST
    }
}

#[async_trait]
impl AttributeCommandRepository for AttributeCommandRepositoryImpl {
    /// Crea un nuevo registro de atributo usando Diesel Async.
    async fn create(
        &self,
        conn: &mut AsyncPgConnection, // Recibe la conexión transaccional async
        entity_id: Uuid,
        data_type_id: Uuid,
        name: &str,
        description: Option<&str>,
        is_required: bool,
        position: i16,
        is_unique: Option<i16>,
        default_value: Option<&str>,
        validation_regex: Option<&str>,
        created_by: Uuid,
    ) -> Result<Uuid, Box<dyn Error + Send + Sync>> {
        debug!("Creando atributo (Diesel Async): name='{}', entity_id='{}'", name, entity_id);

        // Definir los datos a insertar usando tuplas y el DSL de Diesel
        // Asume que 'id' se genera automáticamente (DEFAULT uuid_generate_v4())
        // y 'created_at' / 'updated_at' tienen defaults o triggers.
        let attribute_data = (
            attributes::entity_id.eq(entity_id),
            attributes::data_type_id.eq(data_type_id),
            attributes::name.eq(name),
            attributes::description.eq(description),
            attributes::is_required.eq(is_required),
            attributes::position.eq(position),
            attributes::is_unique.eq(is_unique),
            attributes::default_value.eq(default_value),
            attributes::validation_regex.eq(validation_regex),
            attributes::created_by.eq(Some(created_by)),
            // attributes::status.eq(1), // Establecer estado inicial si es necesario
        );

        // Ejecutar la inserción usando la conexión async
        let inserted_id = diesel::insert_into(attributes::table)
            .values(attribute_data)
            .returning(attributes::id) // Devolver el ID del atributo creado
            .get_result::<Uuid>(conn) // Ejecutar de forma asíncrona
            .await // Esperar el resultado
            .context(format!("Failed to insert attribute '{}' using Diesel Async", name))?; // Contexto

        debug!("Atributo '{}' creado con ID: {}", name, inserted_id);
        Ok(inserted_id)
    }
}

// --- Modelo Diesel (Opcional, si no usas tuplas para insertar) ---
/*
// Debería estar en Infrastructure/Persistence/models/attribute_model.rs
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable)] // Solo Insertable si no necesitas Queryable aquí
#[diesel(table_name = crate::Infrastructure::Persistence::schema::attributes)]
pub struct NewAttributeModel<'a> { // Usar lifetimes si se usan &str
    pub entity_id: Uuid,
    pub data_type_id: Uuid,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub is_required: bool,
    pub position: i16,
    pub is_unique: Option<i16>,
    pub default_value: Option<&'a str>,
    pub validation_regex: Option<&'a str>,
    pub created_by: Option<Uuid>,
    // No incluir id, created_at, updated_at si son generados por la BD
}
*/
