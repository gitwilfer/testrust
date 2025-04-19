use async_trait::async_trait;
use super::data_type::DataType;
use crate::Domain::error::DomainError; // Asumiendo un tipo de error común
use uuid::Uuid;

#[async_trait]
pub trait DataTypeRepository: Send + Sync { // Send + Sync son necesarios para compartir entre threads async
    async fn find_by_name(&self, name: &str) -> Result<Option<DataType>, DomainError>;
    // Podrías añadir otros métodos si son necesarios, ej: find_by_id
    // async fn find_by_id(&self, id: Uuid) -> Result<Option<DataType>, DomainError>;
}
