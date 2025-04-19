use async_trait::async_trait;
use uuid::Uuid;
// TODO: Reemplazar Box<dyn Error> con un enum de error de repositorio específico
use std::error::Error;
use diesel_async::AsyncPgConnection;

#[async_trait]
pub trait LogicalEntityCommandRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut AsyncPgConnection,
        name: &str,
        description: Option<&str>,
        assign_view: Option<&str>,
        created_by: Uuid,
    ) -> Result<Uuid, Box<dyn Error + Send + Sync>>;

    // async fn update(...) -> Result<(), Box<dyn Error + Send + Sync>>; // Para futuras implementaciones
    // async fn delete(...) -> Result<(), Box<dyn Error + Send + Sync>>; // Para futuras implementaciones
}
