use std::future::Future;
use anyhow::Result;
use async_trait::async_trait;
use diesel_async::AsyncPgConnection;
use super::driven::repositories::{
    LogicalEntityCommandRepository,
    LogicalEntityQueryRepository,
    AttributeCommandRepository,
    DataTypeQueryRepository,
    UserQueryRepository,
    UserCommandRepository,
};

#[async_trait]
// El trait que agrupa los repositorios accesibles DENTRO de una UoW
pub trait RepositoryRegistry: Send + Sync {
    // User
    fn user_command_repository(&self) -> &dyn UserCommandRepository; // <-- CAMBIO
    fn user_query_repository(&self) -> &dyn UserQueryRepository;
    // Logical Entity
    fn logical_entity_command_repository(&self) -> &dyn LogicalEntityCommandRepository;
    fn logical_entity_query_repository(&self) -> &dyn LogicalEntityQueryRepository;
    // Attribute & DataType
    fn attribute_command_repository(&self) -> &dyn AttributeCommandRepository;
    fn data_type_query_repository(&self) -> &dyn DataTypeQueryRepository;

    // --- NUEVO MÉTODO ---
    fn get_diesel_async_conn(&mut self) -> &mut AsyncPgConnection; // <-- AÑADIDO
}

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    async fn execute<F, Fut, R>(&self, work: F) -> Result<R>
    where
        F: FnOnce(&mut dyn RepositoryRegistry) -> Fut + Send, // <-- CAMBIO a &mut
        Fut: Future<Output = Result<R>> + Send,
        R: Send;
}
