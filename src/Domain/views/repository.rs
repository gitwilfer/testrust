use async_trait::async_trait;
use crate::Domain::error::DomainError;

#[async_trait]
pub trait ViewRepository: Send + Sync {
    async fn create_or_replace_view(&self, view_name: &str, view_sql: &str) -> Result<(), DomainError>;
}
