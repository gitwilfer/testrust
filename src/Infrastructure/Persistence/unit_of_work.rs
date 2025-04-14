use std::sync::Arc;
use anyhow::Result;
use diesel::Connection;
use diesel::r2d2::{ConnectionManager, Pool };
use diesel::PgConnection;
use sqlx::Postgres;

use crate::Application::ports::repositories::UserRepositoryPort;
use crate::Application::ports::unit_of_work::RepositoryRegistry;
use crate::Domain::entities::user::User;
use crate::Infrastructure::repositories::{UserQueryRepositorySqlx, UserRepositoryImpl};

// Estructura que proporciona acceso a los repositorios dentro de una transacción
pub struct DatabaseRepositoryRegistry {
    // Repositorio original para comandos (Diesel)
    user_repository: Arc<UserRepositoryImpl>,
    
    // Repositorio SQLx para consultas
    user_query_repository: Arc<UserQueryRepositorySqlx>,
}

impl DatabaseRepositoryRegistry {
    pub fn new(
        pool: Arc<Pool<ConnectionManager<PgConnection>>>,
        sqlx_pool: Arc<sqlx::Pool<Postgres>>
    ) -> Result<Self> {
        Ok(Self {
            user_repository: Arc::new(
                UserRepositoryImpl::new().expect("Failed to create UserRepositoryImpl")
            ),
            user_query_repository: Arc::new(
                UserQueryRepositorySqlx::with_pool(sqlx_pool)
            ),
        })
    }
}

impl RepositoryRegistry for DatabaseRepositoryRegistry {
    fn user_repository(&self) -> &dyn UserRepositoryPort {
        // Aquí usamos as_ref() para obtener una referencia al contenido del Arc
        self.user_repository.as_ref()
    }
    // Nuevo método para acceder al repositorio de consulta SQLx
    fn user_query_repository(&self) -> &dyn crate::Application::ports::repositories::UserQueryRepository {
        self.user_query_repository.as_ref()
    }
}

// Implementación actualizada de UnitOfWork con soporte para SQLx
pub struct HybridUnitOfWork {
    diesel_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    sqlx_pool: Arc<sqlx::Pool<Postgres>>,
}

impl HybridUnitOfWork {
    pub fn new(
        diesel_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
        sqlx_pool: Arc<sqlx::Pool<Postgres>>
    ) -> Self {
        Self { diesel_pool, sqlx_pool }
    }
    
    // Método para ejecutar operaciones complejas que requieren ambos tipos de acceso
    pub async fn execute_hybrid_operation<F, R>(&self, operation: F) -> Result<R>
    where
        F: FnOnce(&DatabaseRepositoryRegistry) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R>> + Send>> + Send + 'static,
        R: Send + 'static,
    {
        let registry = DatabaseRepositoryRegistry::new(
            self.diesel_pool.clone(),
            self.sqlx_pool.clone()
        )?;
        
        // Al usar registros separados, no necesitamos una transacción Diesel para consultas
        operation(&registry).await
    }
    
    // Método para operaciones que requieren transacciones Diesel
    pub async fn execute_with_transaction<F, R>(&self, operation: F) -> Result<R>
    where
        F: FnOnce(&DatabaseRepositoryRegistry) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R>> + Send>> + Send + 'static,
        R: Send + 'static,
    {
        let registry = DatabaseRepositoryRegistry::new(
            self.diesel_pool.clone(),
            self.sqlx_pool.clone()
        )?;
        
        let mut conn = self.diesel_pool.get()?;
        
        conn.transaction(|_conn| {
            let runtime = tokio::runtime::Handle::current();
            runtime.block_on(async {
                operation(&registry).await
            })
        })
    }
    
    // Ejemplos de métodos específicos
    
    // Ejemplo de operación de solo lectura con SQLx
    pub async fn find_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let registry = DatabaseRepositoryRegistry::new(
            self.diesel_pool.clone(),
            self.sqlx_pool.clone()
        )?;
        
        registry.user_query_repository().find_by_username(username).await
    }
    
    // Ejemplo de operación de escritura con Diesel
    pub async fn create_user(&self, user: User) -> Result<User> {
        let mut conn = self.diesel_pool.get()?;
        
        conn.transaction(|_conn| {
            let registry = DatabaseRepositoryRegistry::new(
                self.diesel_pool.clone(),
                self.sqlx_pool.clone()
            ).expect("Failed to create repository registry");
            
            let runtime = tokio::runtime::Handle::current();
            runtime.block_on(async {
                registry.user_repository().create(user.clone()).await
            })
        })
    }
}

// Implementación concreta de UnitOfWork que usa DatabaseManager
pub struct DatabaseUnitOfWork {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    sqlx_pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

impl DatabaseUnitOfWork {
    pub fn new(
        pool: Arc<Pool<ConnectionManager<PgConnection>>>,
        sqlx_pool: Arc<sqlx::Pool<sqlx::Postgres>>
    ) -> Self {
        Self { pool, sqlx_pool }
    }
    
    // Método para ejecutar operaciones en transacción
    pub async fn execute_create_user(&self, user: User) -> Result<User> {
        let mut conn = self.pool.get()?;
        
        conn.transaction(|_conn| {
            let registry = DatabaseRepositoryRegistry::new(
                self.pool.clone(),
                self.sqlx_pool.clone()
            ).expect("Failed to create repository registry");
            
            // Creamos el usuario
            let user_repo = registry.user_repository();
            
            // Necesitamos un bloque para manejar el async en contexto síncrono
            let runtime = tokio::runtime::Handle::current();
            runtime.block_on(async {
                user_repo.create(user.clone()).await
            })
        })
    }
    
    // Método para ejecutar actualización de usuario en transacción
    pub async fn execute_update_user(&self, user: User) -> Result<User> {
        let mut conn = self.pool.get()?;
        
        conn.transaction(|_conn| {
            let registry = DatabaseRepositoryRegistry::new(
                self.pool.clone(),
                self.sqlx_pool.clone()
            ).expect("Failed to create repository registry");
            
            let user_repo = registry.user_repository();
            
            let runtime = tokio::runtime::Handle::current();
            runtime.block_on(async {
                user_repo.update(user.clone()).await
            })
        })
    }
    
    // Puedes añadir más métodos para operaciones específicas según sea necesario
}