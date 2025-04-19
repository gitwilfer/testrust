use async_trait::async_trait;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
// TODO: Reemplazar Box<dyn Error> con un enum de error de repositorio específico
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)] // La implementación añadirá sqlx::FromRow
pub struct LogicalEntityDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub assign_view: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>, // Ajustar si SQLx mapea a NaiveDateTime
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<DateTime<Utc>>, // Ajustar si SQLx mapea a NaiveDateTime
    pub status: i16,
}

#[async_trait]
pub trait LogicalEntityQueryRepository: Send + Sync {
    async fn exists_by_name(
        &self,
        name: &str
    ) -> Result<bool, Box<dyn Error + Send + Sync>>;

    // async fn find_by_id(...) -> Result<Option<LogicalEntityDto>, Box<dyn Error + Send + Sync>>; // Para futuras implementaciones
    // async fn find_all(...) -> Result<Vec<LogicalEntityDto>, Box<dyn Error + Send + Sync>>; // Para futuras implementaciones
}
