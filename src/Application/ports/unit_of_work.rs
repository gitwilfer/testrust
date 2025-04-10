// src/application/ports/unit_of_work.rs
use std::future::Future;
use std::pin::Pin;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait RepositoryRegistry: Send + Sync {
    fn user_repository(&self) -> &dyn crate::Application::ports::repositories::UserRepositoryPort;
    // Añadir otros repositorios según sea necesario
}

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    async fn execute<F, Fut, R>(&self, work: F) -> Result<R>
    where
        F: FnOnce(&dyn RepositoryRegistry) -> Fut + Send + 'static,
        Fut: Future<Output = Result<R>> + Send + 'static,
        R: Send + 'static;
}