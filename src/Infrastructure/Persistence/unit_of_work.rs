use anyhow::{Result, Context, anyhow};
use async_trait::async_trait;
// --- Diesel Async Imports ---
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager,
    AsyncPgConnection,
    RunQueryDsl,
    scoped_futures::ScopedFutureExt,
    pooled_connection::bb8::Pool as AsyncDieselPool,
};
// --- SQLx Imports ---
use sqlx::{PgPool as SqlxPool};
// --- Standard Imports ---
use std::future::Future;
use std::sync::Arc;
use log::{error, debug};

// --- Importar Traits de Ports ---
use crate::Application::ports::unit_of_work::{UnitOfWork, RepositoryRegistry};
use crate::Application::ports::driven::repositories::{
    // User Repositories
    UserQueryRepository, UserCommandRepository, // UserRepositoryPort eliminado (no usado)
    // Logical Entity Repositories
    LogicalEntityCommandRepository, LogicalEntityQueryRepository,
    // Attribute & DataType Repositories
    AttributeCommandRepository, DataTypeQueryRepository, // <--- Asegurarse que estén importados
};

// --- Importar Implementaciones de Repositorios ---
use crate::Infrastructure::repositories::{
    // UserRepositoryImpl, 
    UserQueryRepositorySqlx,
    UserCommandRepositoryImpl,
    // Logical Entity Repositories
    LogicalEntityCommandRepositoryImpl, LogicalEntityQueryRepositoryImpl,
    // Attribute & DataType Repositories
    AttributeCommandRepositoryImpl, DataTypeQueryRepositoryImpl, // <--- Asegurarse que estén importados
};

// --- Implementación del Registro (Contextual a la Transacción Async) ---
struct TransactionalRepositoryRegistry<'conn> {
    diesel_tx_conn: &'conn mut AsyncPgConnection, // Conexión Diesel Async Transaccional
    user_query_repo: Arc<UserQueryRepositorySqlx>,
    le_query_repo: Arc<LogicalEntityQueryRepositoryImpl>,
    dt_query_repo: Arc<DataTypeQueryRepositoryImpl>, // <--- Añadido para DataType
}

impl<'conn> TransactionalRepositoryRegistry<'conn> {
    fn new(
        diesel_tx_conn: &'conn mut AsyncPgConnection,
        user_query_repo: Arc<UserQueryRepositorySqlx>,
        le_query_repo: Arc<LogicalEntityQueryRepositoryImpl>,
        dt_query_repo: Arc<DataTypeQueryRepositoryImpl>, // <--- Añadido
    ) -> Self {
        Self {
            diesel_tx_conn,
            user_query_repo,
            le_query_repo,
            dt_query_repo, // <--- Añadido
        }
    }

    fn conn(&mut self) -> &mut AsyncPgConnection {
        self.diesel_tx_conn
    }
}

// Implementa el trait del Port RepositoryRegistry
impl<'conn> RepositoryRegistry for TransactionalRepositoryRegistry<'conn> {

    fn user_command_repository(&self) -> &dyn UserCommandRepository {
        &UserCommandRepositoryImpl
    }

    fn user_query_repository(&self) -> &dyn UserQueryRepository {
        self.user_query_repo.as_ref()
    }

    // --- Logical Entity Repos ---
    fn logical_entity_command_repository(&self) -> &dyn LogicalEntityCommandRepository {
         &LogicalEntityCommandRepositoryImpl
    }
    fn logical_entity_query_repository(&self) -> &dyn LogicalEntityQueryRepository {
        self.le_query_repo.as_ref()
    }

    // --- Attribute & DataType Repos ---
    fn attribute_command_repository(&self) -> &dyn AttributeCommandRepository { // <--- COMPLETADO
        &AttributeCommandRepositoryImpl
    }
    fn data_type_query_repository(&self) -> &dyn DataTypeQueryRepository { // <--- COMPLETADO
        self.dt_query_repo.as_ref()
    }
    fn get_diesel_async_conn(&mut self) -> &mut AsyncPgConnection { // <-- AÑADIDO
        self.conn()
    }
}

// --- Implementación ÚNICA y CORRECTA de UnitOfWork (Usando diesel-async) ---
pub struct DieselAsyncUnitOfWork {
    diesel_async_pool: Arc<AsyncDieselPool<AsyncPgConnection>>,
    sqlx_pool: Arc<SqlxPool>,
    // --- Pre-instanciar Repositorios de Consulta ---
    user_query_repo: Arc<UserQueryRepositorySqlx>,
    le_query_repo: Arc<LogicalEntityQueryRepositoryImpl>,
    dt_query_repo: Arc<DataTypeQueryRepositoryImpl>, // <--- Añadido
}

impl DieselAsyncUnitOfWork {
    pub fn new(
        diesel_async_pool: Arc<AsyncDieselPool<AsyncPgConnection>>,
        sqlx_pool: Arc<SqlxPool>
    ) -> Self {
        // Crear instancias de los repositorios de consulta aquí
        let user_query_repo = Arc::new(UserQueryRepositorySqlx::with_pool(sqlx_pool.clone()));
        let le_query_repo = Arc::new(LogicalEntityQueryRepositoryImpl::new(sqlx_pool.clone()));
        let dt_query_repo = Arc::new(DataTypeQueryRepositoryImpl::new(sqlx_pool.clone())); // <--- Añadido
        Self {
            diesel_async_pool,
            sqlx_pool,
            user_query_repo,
            le_query_repo,
            dt_query_repo,
        }
    }
}

#[async_trait]
impl UnitOfWork for DieselAsyncUnitOfWork {
    async fn execute<F, Fut, R>(&self, work: F) -> Result<R>
    where
        F: FnOnce(&dyn RepositoryRegistry) -> Fut + Send,
        Fut: Future<Output = Result<R>> + Send,
        R: Send,
    {
        debug!("Iniciando Unidad de Trabajo Asíncrona (Diesel Async)");
        let mut conn = self.diesel_async_pool.get().await
            .context("Failed to get async Diesel connection from pool")?;

        let result = conn.transaction::<_, R, diesel::result::Error>(|tx_conn| {
            async move {
                debug!("Dentro de la transacción Diesel Async");
                // Crear el registro contextual, pasando la conexión y los repos de consulta
                let registry = TransactionalRepositoryRegistry::new(
                    tx_conn,
                    self.user_query_repo.clone(),
                    self.le_query_repo.clone(),
                    self.dt_query_repo.clone(), // <--- Añadido
                );

                // Ejecutar la clausura del caso de uso
                let operation_result = work(&registry).await;

                // Mapear resultado para commit/rollback
                match operation_result {
                    Ok(value) => {
                        debug!("Operación UoW exitosa, preparando commit");
                        Ok(value) // COMMIT
                    },
                    Err(app_err) => {
                        error!("Error dentro de la operación UoW, iniciando rollback: {:?}", app_err);
                        Err(diesel::result::Error::RollbackTransaction) // ROLLBACK
                    }
                }
            }
            .scope_boxed()
        }).await;

        debug!("Unidad de Trabajo Asíncrona completada");

        // Mapear error final
        result.map_err(|diesel_err| {
            error!("Error en la transacción Diesel: {:?}", diesel_err);
            anyhow!("Transaction failed: {}", diesel_err)
        })
    }
}


