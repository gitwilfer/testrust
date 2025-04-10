// src/Infrastructure/repositories/sqlx_repository_base.rs

use anyhow::{Result, anyhow};
use sqlx::{Pool, Postgres, Transaction};
use std::sync::Arc;

use crate::Application::services::get_database_for_entity;
use crate::Infrastructure::Persistence::sqlx_database;

// Repositorio base con funcionalidad común para todos los repositorios SQLx
pub struct SqlxRepositoryBase {
    pool: Arc<sqlx::Pool<Postgres>>,
    entity_name: String,
}

impl SqlxRepositoryBase {
    // Crear nuevo repositorio base para una entidad específica
    pub async fn new(entity_name: &str) -> Result<Self> {
        let db_name = get_database_for_entity(entity_name);
        let pool = sqlx_database::get_pool(&db_name).await?;
        
        Ok(Self {
            pool: Arc::new(pool),
            entity_name: entity_name.to_string(),
        })
    }
    
    // Crear desde un pool existente
    pub fn with_pool(pool: Arc<Pool<Postgres>>, entity_name: &str) -> Self {
        Self {
            pool,
            entity_name: entity_name.to_string(),
        }
    }
    
    // Obtener referencia al pool
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
    
    // Obtener el nombre de la entidad
    pub fn entity_name(&self) -> &str {
        &self.entity_name
    }
    
    // Iniciar una transacción
    pub async fn begin_transaction(&self) -> Result<Transaction<'_, Postgres>> {
        Ok(self.pool.begin().await?)
    }
    
    // Ejecutar operación en una transacción
    pub async fn execute_in_transaction<F, R>(&self, operation: F) -> Result<R>
    where
        F: for<'a> FnOnce(&'a mut Transaction<'_, Postgres>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R>> + Send + 'a>>,
        R: Send + 'static,
    {
        let mut tx = self.pool.begin().await?;
        let result = operation(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
    }
}