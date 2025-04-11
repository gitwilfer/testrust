// src/Application/ports/repositories.rs
use std::future::Future;
// use std::pin::Pin;
use anyhow::Result;
use uuid::Uuid;
use crate::Domain::entities::user::User;
use async_trait::async_trait;



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

// Nuevo trait para operaciones transaccionales
#[async_trait]
pub trait TransactionalUserRepository: UserRepositoryPort {
    async fn transaction<F, Fut, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&dyn UserRepositoryPort) -> Fut + Send + 'static,
        Fut: Future<Output = Result<R>> + Send + 'static,
        R: Send + 'static;
        
    // Métodos de conveniencia para operaciones comunes en transacción
    async fn create_in_transaction(&self, user: User) -> Result<User>;
    async fn update_in_transaction(&self, user: User) -> Result<User>;
}

/// Puerto para el servicio de autenticación
#[async_trait]
pub trait AuthServicePort: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String>;
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool>;
    async fn generate_token(&self, user_id: Uuid) -> Result<String>;
    async fn validate_token(&self, token: &str) -> Result<Uuid>;
}




// Query (consultas) - Operaciones de solo lectura
#[async_trait]
pub trait UserQueryRepository: Send + Sync {
    // Configurar la base de datos para consultas
    fn set_database(&mut self, database_name: &str);
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn find_all(&self) -> Result<Vec<User>>;
}

// Command (comandos) - Operaciones de escritura

#[async_trait]
pub trait UserCommandRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User>;
    async fn update(&self, user: User) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}