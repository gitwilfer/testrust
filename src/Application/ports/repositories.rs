// src/Application/ports/repositories.rs
use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;
use std::future::Future;
use std::pin::Pin;
use crate::domain::entities::user::User;

/// Puerto para el repositorio de usuarios - operaciones básicas
#[async_trait]
pub trait UserRepositoryPort: Send + Sync {
    async fn create(&self, user: User) -> Result<User>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn update(&self, user: User) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn find_all(&self) -> Result<Vec<User>>;
}

/// Puerto para el servicio de autenticación
#[async_trait]
pub trait AuthServicePort: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String>;
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool>;
    async fn generate_token(&self, user_id: Uuid) -> Result<String>;
    async fn validate_token(&self, token: &str) -> Result<Uuid>;
}