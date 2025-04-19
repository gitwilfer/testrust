use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

/// Puerto para el servicio de autenticación
#[async_trait]
pub trait AuthServicePort: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String>;
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool>;
    async fn generate_token(&self, user_id: Uuid) -> Result<String>;
    async fn validate_token(&self, token: &str) -> Result<Uuid>;
}
