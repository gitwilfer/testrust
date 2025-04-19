use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;
use anyhow::{Result, Context}; // Añadir Context
use std::error::Error; // Mantener si se usa en firmas de trait

use crate::Application::ports::driven::repositories::{LogicalEntityQueryRepository, LogicalEntityDto};


#[derive(Clone)] // Añadir Clone si se necesita
pub struct LogicalEntityQueryRepositoryImpl {
    pool: Arc<Pool<Postgres>>,
}

impl LogicalEntityQueryRepositoryImpl {
    /// Constructor Preferido: Recibe el pool (Inyección de Dependencias).
    pub fn with_pool(pool: Arc<Pool<Postgres>>) -> Self { // Renombrado/Asegurado
        Self { pool }
    }
}

#[async_trait]
impl LogicalEntityQueryRepository for LogicalEntityQueryRepositoryImpl {
    async fn exists_by_name(&self, name: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let result = sqlx::query("SELECT EXISTS(SELECT 1 FROM logical_entities WHERE name = $1 AND status = 1)")
            .bind(name)
            .fetch_one(&*self.pool) // Usar &* para obtener &Pool<Postgres> de Arc
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?; // Mapear error

        Ok(result.try_get(0).unwrap_or(false))
    }

    // Implementar otros métodos del trait si existen...
    // async fn find_by_id(...) -> Result<Option<LogicalEntityDto>, Box<dyn Error + Send + Sync>> { ... }
    // async fn find_all(...) -> Result<Vec<LogicalEntityDto>, Box<dyn Error + Send + Sync>> { ... }
}
