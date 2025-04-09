// src/infrastructure/persistence/unit_of_work.rs
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use anyhow::Result;
use async_trait::async_trait;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::Connection;

use crate::application::ports::unit_of_work::{UnitOfWork, RepositoryRegistry};
use crate::application::ports::repositories::UserRepositoryPort;
use crate::infrastructure::repositories::user_repository_impl::UserRepositoryImpl;

pub struct DatabaseRepositoryRegistry {
    user_repository: UserRepositoryImpl,
}

impl DatabaseRepositoryRegistry {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        Self {
            user_repository: Arc::new(UserRepositoryImpl::new(pool).unwrap()),
        }
    }
}

impl RepositoryRegistry for DatabaseRepositoryRegistry {
    fn user_repository(&self) -> &dyn UserRepositoryPort {
        &*self.user_repository
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
        
        conn.transaction::<_, anyhow::Error, _>(|conn| {
            let registry = DatabaseRepositoryRegistry::new(conn);
            let future = work(&registry);
            
            // Ejecutar el future en el runtime actual
            tokio::runtime::Handle::current().block_on(future)
        })
    }
}