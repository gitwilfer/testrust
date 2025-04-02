use anyhow::Result;
use async_trait::async_trait;
use std::future::Future;
use std::sync::Arc;

use crate::application::ports::repositories::{UserRepositoryPort, TransactionalUserRepository};

/// Helper para ejecutar operaciones en transacciones
pub struct TransactionHelper {
    repo: Arc<dyn TransactionalUserRepository>,
}

impl TransactionHelper {
    pub fn new(repo: Arc<dyn TransactionalUserRepository>) -> Self {
        Self { repo }
    }
    
    /// Ejecuta una operación dentro de una transacción
    pub async fn execute<F, Fut, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&dyn UserRepositoryPort) -> Fut + Send + 'static,
        Fut: Future<Output = Result<R>> + Send + 'static,
        R: Send + 'static,
    {
        self.repo.transaction(f).await
    }
}