// src/Infrastructure/persistence/repositories/data_type_repository.rs (ejemplo)
use async_trait::async_trait;
use sqlx::PgPool;
use crate::Domain::data_types::{
    data_type::DataType,
    repository::DataTypeRepository,
};
use crate::Domain::error::DomainError; // O un error específico de Infraestructura
use uuid::Uuid;

// Estructura para la implementación del repositorio
#[derive(Clone)] // Clone es útil para la inyección de dependencias
pub struct SqlxDataTypeRepository {
    pool: PgPool,
}

// Constructor
impl SqlxDataTypeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DataTypeRepository for SqlxDataTypeRepository {
    async fn find_by_name(&self, name: &str) -> Result<Option<DataType>, DomainError> {
        // Usar query_as! para mapear directamente a la struct DataType
        // Asegúrate que la struct DataType tenga #[derive(sqlx::FromRow)] o que los nombres coincidan
        // Incluir status = 1 para obtener solo tipos activos
        let result = sqlx::query_as!(
            DataType,
            "SELECT id, name FROM data_types WHERE name = $1 AND status = 1",
            name
        )
        .fetch_optional(&self.pool) // fetch_optional devuelve Option<T>
        .await;

        match result {
            Ok(data_type_option) => Ok(data_type_option),
            Err(sqlx_error) => {
                // Loggear el error real sqlx_error
                eprintln!("Database error fetching data type by name: {:?}", sqlx_error);
                // Mapear a un error de dominio/aplicación genérico
                Err(DomainError::DatabaseError(sqlx_error.to_string()))
            }
        }
    }

    /* // Implementación de ejemplo para find_by_id
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DataType>, DomainError> {
         let result = sqlx::query_as!(
            DataType,
            "SELECT id, name FROM data_types WHERE id = $1 AND status = 1",
            id
        )
        .fetch_optional(&self.pool)
        .await;
         match result {
            Ok(data_type_option) => Ok(data_type_option),
            Err(sqlx_error) => {
                eprintln!("Database error fetching data type by id: {:?}", sqlx_error);
                Err(DomainError::DatabaseError(sqlx_error.to_string()))
            }
        }
    }
    */
}

// Nota: Asegúrate de tener un tipo de error adecuado (DomainError o similar)
// y que maneje errores de base de datos.
