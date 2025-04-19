use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;
use uuid::Uuid;
use anyhow::{Result, Context}; // Añadir Context
use std::error::Error; // Mantener si se usa en firmas de trait

use crate::Application::ports::driven::repositories::DataTypeQueryRepository;

#[derive(Clone)] // Añadir Clone si se necesita
pub struct DataTypeQueryRepositoryImpl {
    pool: Arc<Pool<Postgres>>,
}

impl DataTypeQueryRepositoryImpl {
    /// Constructor Preferido: Recibe el pool (Inyección de Dependencias).
    pub fn with_pool(pool: Arc<Pool<Postgres>>) -> Self { // Renombrado/Asegurado
        Self { pool }
    }
}

#[async_trait]
impl DataTypeQueryRepository for DataTypeQueryRepositoryImpl {
    async fn find_id_by_name(&self, name: &str) -> Result<Option<Uuid>, Box<dyn Error + Send + Sync>> {
        let result = sqlx::query("SELECT id FROM data_types WHERE LOWER(name) = LOWER($1) AND status = 1")
            .bind(name)
            .fetch_optional(&*self.pool) // Usar fetch_optional y &*
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?; // Mapear error

        // Mapear el Option<Row> a Option<Uuid>
        Ok(result.map(|row| row.get("id")))
    }
}
