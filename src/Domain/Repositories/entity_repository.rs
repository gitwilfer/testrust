use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result as AnyhowResult; // <-- Añadido y alias usado

use crate::Domain::entities::Entity;

#[async_trait]
pub trait EntityRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> AnyhowResult<Option<Entity>>; // <-- Cambiado
    async fn find_by_name(&self, name: &str) -> AnyhowResult<Option<Entity>>; // <-- Cambiado
    async fn find_all(&self) -> AnyhowResult<Vec<Entity>>; // <-- Cambiado
    async fn create(&self, entity: Entity) -> AnyhowResult<Entity>; // <-- Cambiado
    async fn update(&self, entity: Entity) -> AnyhowResult<Entity>; // <-- Cambiado
    async fn delete(&self, id: Uuid) -> AnyhowResult<()>; // <-- Cambiado
}
