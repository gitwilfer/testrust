// Este archivo será marcado como deprecated y eventualmente eliminado

use crate::domain::entities::user::User;
use anyhow::Result;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

#[deprecated(
    since = "0.1.0",
    note = "Por favor use UserRepositoryPort de la capa de aplicación"
)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn update(&self, user: User) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn find_all(&self) -> Result<Vec<User>>;
    
    // Método para ejecutar operaciones dentro de una transacción
    async fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(Box<dyn UserRepository + '_>) -> Pin<Box<dyn Future<Output = Result<R>> + Send + '_>> + Send,
        R: Send + 'static;
}