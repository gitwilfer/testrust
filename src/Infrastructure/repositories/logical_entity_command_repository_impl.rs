use async_trait::async_trait;
use diesel::prelude::*; // Importar Selectable, Insertable, etc.
use diesel_async::{AsyncPgConnection, RunQueryDsl}; // Usar RunQueryDsl de diesel_async
use std::error::Error;
use uuid::Uuid;
use anyhow::Context; // Para añadir contexto a errores

// Importar el trait del Port
use crate::Application::ports::driven::repositories::LogicalEntityCommandRepository;
// Importar el modelo Diesel y el schema
use crate::Infrastructure::Persistence::models::LogicalEntityModel; // Asume que existe y tiene los derives necesarios
use crate::Infrastructure::Persistence::schema::logical_entities;

// La implementación puede ser un struct vacío (Zero-Sized Type) si no tiene estado propio.
#[derive(Clone, Copy)] // Añadir derives si es ZST
pub struct LogicalEntityCommandRepositoryImpl;

impl LogicalEntityCommandRepositoryImpl {
    // Constructor simple si es ZST
    pub fn new() -> Self {
        Self
    }
    // Si tuviera estado (ej: un logger), se pasaría aquí
}

#[async_trait]
impl LogicalEntityCommandRepository for LogicalEntityCommandRepositoryImpl {
    async fn create(
        &self,
        conn: &mut AsyncPgConnection, // Recibe la conexión transaccional async
        name: &str,
        description: Option<&str>,
        assign_view: Option<&str>,
        created_by: Uuid,
    ) -> Result<Uuid, Box<dyn Error + Send + Sync>> {
        // Crear la instancia del modelo Diesel para insertar
        // Asumiendo que LogicalEntityModel implementa Insertable
        // y que los campos como id, created_at, etc., son manejados por la BD o defaults.
        let new_entity_data = (
            logical_entities::name.eq(name),
            logical_entities::description.eq(description),
            logical_entities::assign_view.eq(assign_view),
            logical_entities::created_by.eq(Some(created_by)),
            // Añadir otros campos necesarios para la inserción
            // logical_entities::status.eq(1), // Ejemplo: estado inicial
        );

        // Ejecutar la inserción usando la conexión async y RunQueryDsl de diesel_async
        let inserted_id = diesel::insert_into(logical_entities::table)
            .values(&new_entity_data)
            .returning(logical_entities::id) // Devuelve el ID generado
            .get_result::<Uuid>(conn) // Usar get_result (async)
            .await // Esperar el resultado async
            .context("Failed to insert new logical entity using Diesel Async")?; // Añadir contexto con anyhow

        Ok(inserted_id)
    }

    // Implementar update, delete de forma similar usando conn.execute() o .get_result() async
}

// --- Modelo Diesel (Ejemplo, debería estar en Infrastructure/Persistence/models/) ---
/*
#[derive(Queryable, Selectable, Insertable, Debug, Clone)]
#[diesel(table_name = crate::Infrastructure::Persistence::schema::logical_entities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LogicalEntityModel {
    #[diesel(deserialize_as = Uuid)] // Asegúrate que Diesel sepa manejar Uuid
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub assign_view: Option<String>,
    #[diesel(deserialize_as = Option<Uuid>)]
    pub created_by: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime, // Diesel async a menudo usa NaiveDateTime
    #[diesel(deserialize_as = Option<Uuid>)]
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub status: i16,
    // ... otros campos ...
}
*/
