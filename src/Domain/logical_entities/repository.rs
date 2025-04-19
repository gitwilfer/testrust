// src/Domain/logical_entities/repository.rs

use async_trait::async_trait;
use uuid::Uuid;
use crate::Domain::logical_entities::logical_entity::LogicalEntity; // Asegúrate que la ruta sea correcta

// Definir un tipo de error genérico para el repositorio.
// Idealmente, se definiría un enum de errores más específico.
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Logical entity not found with ID: {0}")]
    NotFoundById(Uuid),
    #[error("Logical entity not found with name: {0}")]
    NotFoundByName(String),
    #[error("Logical entity with name '{0}' already exists")]
    AlreadyExists(String),
    #[error("Database error: {0}")]
    DatabaseError(String), // Para errores genéricos de BD
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

// Alias para el resultado del repositorio
pub type RepositoryResult<T> = Result<T, RepositoryError>;

/// Trait que define las operaciones para persistir y recuperar LogicalEntities.
/// Se utiliza `async_trait` para permitir implementaciones asíncronas.
#[async_trait]
pub trait LogicalEntityRepository: Send + Sync {
    /// Guarda (inserta o actualiza) una LogicalEntity en la base de datos.
    /// Devuelve un error si ya existe una entidad con el mismo nombre al intentar insertar.
    async fn save(&self, entity: &LogicalEntity) -> RepositoryResult<()>;

    /// Busca una LogicalEntity por su ID.
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<LogicalEntity>;

    /// Busca una LogicalEntity por su nombre único.
    async fn find_by_name(&self, name: &str) -> RepositoryResult<LogicalEntity>;

    /// Verifica si existe una LogicalEntity con el nombre dado.
    async fn exists_by_name(&self, name: &str) -> RepositoryResult<bool>;

    // Podrían añadirse otros métodos según sea necesario, como:
    // async fn find_all(&self, ...) -> RepositoryResult<Vec<LogicalEntity>>;
    // async fn delete(&self, id: Uuid) -> RepositoryResult<()>;
}