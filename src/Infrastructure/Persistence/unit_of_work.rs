// src/infrastructure/persistence/unit_of_work.rs
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use anyhow::Result;
use async_trait::async_trait;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

use crate::application::ports::unit_of_work::{UnitOfWork, RepositoryRegistry};
use crate::application::ports::repositories::UserRepositoryPort;
use crate::infrastructure::repositories::user_repository_impl::UserRepositoryImpl;

pub struct DatabaseRepositoryRegistry<'a> {
    conn: &'a mut diesel::PgConnection,
    user_repository: UserRepositoryImpl,
}

impl<'a> DatabaseRepositoryRegistry<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self {
            conn,
            user_repository: UserRepositoryImpl::with_connection(conn),
        }
    }
}

impl<'a> RepositoryRegistry for DatabaseRepositoryRegistry<'a> {
    fn user_repository(&self) -> &dyn UserRepositoryPort {
        &self.user_repository
    }
}

pub struct DatabaseUnitOfWork {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl DatabaseUnitOfWork {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnitOfWork for DatabaseUnitOfWork {
    async fn execute<F, Fut, R>(&self, work: F) -> Result<R>
    where
        F: FnOnce(&dyn RepositoryRegistry) -> Fut + Send + 'static,
        Fut: Future<Output = Result<R>> + Send + 'static,
        R: Send + 'static,
    {
        let mut conn = self.pool.get()?;
        
        conn.transaction(|conn| {
            let registry = DatabaseRepositoryRegistry::new(conn);
            let future = work(&registry);
            
            // Ejecutar el future en el runtime actual
            tokio::runtime::Handle::current().block_on(future)
        })
    }
}