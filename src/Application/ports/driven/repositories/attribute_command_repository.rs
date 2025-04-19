use async_trait::async_trait;
use uuid::Uuid;
use std::error::Error;
use diesel_async::AsyncPgConnection; // Necesita la conexión async

/// Driven Port: Define las operaciones de escritura para Atributos.
/// Se espera implementación con Diesel Async dentro de una transacción UoW.
#[async_trait]
pub trait AttributeCommandRepository: Send + Sync {
    /// Crea un nuevo registro de atributo.
    async fn create(
        &self,
        conn: &mut AsyncPgConnection, // Recibe la conexión transaccional
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
    ) -> Result<Uuid, Box<dyn Error + Send + Sync>>; // Devuelve el ID del nuevo atributo
}
