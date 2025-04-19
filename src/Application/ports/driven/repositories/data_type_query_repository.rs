// src/Application/Ports/driven/repositories/data_type_query_repository.rs
use async_trait::async_trait;
use uuid::Uuid;
use std::error::Error;

/// Driven Port: Define cómo buscar información de Tipos de Datos.
/// Se espera implementación con SQLx.
#[async_trait]
pub trait DataTypeQueryRepository: Send + Sync {
    /// Busca el ID de un tipo de dato por su nombre (case-insensitive).
    async fn find_id_by_name(
        &self,
        name: &str
    ) -> Result<Option<Uuid>, Box<dyn Error + Send + Sync>>;
}
