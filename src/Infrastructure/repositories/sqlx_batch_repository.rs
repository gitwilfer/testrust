use anyhow::{Result, anyhow};
//use futures::{stream, StreamExt};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
//use uuid::Uuid;

use crate::Domain::entities::user::User; // Asumiendo que es para User
use crate::Infrastructure::repositories::sqlx_repository_base::SqlxRepositoryBase;

/// Repositorio para operaciones en lote
pub struct BatchRepository {
    base: SqlxRepositoryBase,
    batch_size: usize,
}

impl BatchRepository {

    /// Constructor Preferido: Recibe el pool (Inyección de Dependencias).
    pub fn with_pool(pool: Arc<Pool<Postgres>>, entity_name: &str, batch_size: usize) -> Self {
        Self {
            base: SqlxRepositoryBase::with_pool(pool, entity_name),
            batch_size,
        }
    }

    pub async fn bulk_insert_users(&self, users: Vec<User>) -> Result<Vec<User>> { /* ... */ }
    async fn insert_user_batch(&self, users: Vec<User>) -> Result<Vec<User>> { /* ... */ }
    pub async fn bulk_update_users(&self, users: Vec<User>, concurrency: usize) -> Result<Vec<User>> { /* ... */ }
}
