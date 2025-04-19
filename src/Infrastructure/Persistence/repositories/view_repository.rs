use async_trait::async_trait;
use sqlx::PgPool;
use crate::Domain::error::DomainError;
// Quitar la línea de abajo si no se define el trait en Domain
// use crate::Domain::views::repository::ViewRepository;

#[derive(Clone)]
pub struct SqlxViewRepository {
    pool: PgPool,
}

impl SqlxViewRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Método directo si no se usa el trait
    pub async fn execute_create_or_replace_view(&self, view_sql: &str) -> Result<(), DomainError> {
        sqlx::query(view_sql)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                eprintln!("Failed to create/replace view: {}", e);
                DomainError::DatabaseError(format!("Failed to execute DDL for view: {}", e))
            })?;
        Ok(())
    }
}

// Implementación del trait (si se definió en Domain)
/*
#[async_trait]
impl ViewRepository for SqlxViewRepository {
    async fn create_or_replace_view(&self, _view_name: &str, view_sql: &str) -> Result<(), DomainError> {
        // El nombre de la vista ya está en el SQL
        self.execute_create_or_replace_view(view_sql).await
    }
}
*/
