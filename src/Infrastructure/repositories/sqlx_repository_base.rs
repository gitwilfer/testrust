use anyhow::{Result, Context}; // Asegúrate que Context esté importado
use sqlx::{Pool, Postgres, Transaction};
use std::sync::Arc;


/// Requiere que se le inyecte el pool de conexiones SQLx.
#[derive(Clone)] // Añadir Clone si se va a clonar el base
pub struct SqlxRepositoryBase {
    pool: Arc<sqlx::Pool<Postgres>>,
    entity_name: String,
}

impl SqlxRepositoryBase {
    /// Constructor Preferido: Recibe el pool (Inyección de Dependencias).
    pub fn with_pool(pool: Arc<Pool<Postgres>>, entity_name: &str) -> Self {
        Self {
            pool,
            entity_name: entity_name.to_string(),
        }
    }

    /// Obtener referencia al pool.
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    /// Obtener el nombre de la entidad.
    pub fn entity_name(&self) -> &str {
        &self.entity_name
    }

    /// Iniciar una transacción.
    pub async fn begin_transaction(&self) -> Result<Transaction<'_, Postgres>> {
        self.pool.begin().await
            .with_context(|| format!("Failed to begin SQLx transaction for entity '{}'", self.entity_name))
    }

    /// Ejecutar operación en una transacción.
    pub async fn execute_in_transaction<F, R>(&self, operation: F) -> Result<R>
    where
        F: for<'a> FnOnce(&'a mut Transaction<'_, Postgres>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R>> + Send + 'a>>,
        R: Send + 'static,
    {
        let mut tx = self.begin_transaction().await?;
        // Ejecutar la operación pasada como closure
        let result = operation(&mut tx).await;

        // Hacer commit o rollback basado en el resultado de la operación
        match result {
            Ok(value) => {
                tx.commit().await
                  .with_context(|| format!("Failed to commit SQLx transaction for entity '{}'", self.entity_name))?;
                Ok(value)
            }
            Err(e) => {
                // Intentar rollback, aunque el error original es más importante
                if let Err(rollback_err) = tx.rollback().await {
                    log::error!("Failed to rollback SQLx transaction for entity '{}' after error: {}", self.entity_name, rollback_err);
                }
                // Devolver el error original de la operación
                Err(e)
            }
        }
    }
}
