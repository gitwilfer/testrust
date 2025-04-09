use std::sync::Arc;
use anyhow::Result;
use diesel::Connection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

use crate::application::ports::repositories::UserRepositoryPort;
use crate::application::ports::unit_of_work::RepositoryRegistry;
use crate::domain::entities::user::User;
use crate::infrastructure::repositories::UserRepositoryImpl;

// Estructura que proporciona acceso a los repositorios dentro de una transacción
pub struct DatabaseRepositoryRegistry {
    // Guardamos directamente el repositorio implementado
    user_repository: Arc<UserRepositoryImpl>,
}

impl DatabaseRepositoryRegistry {
    // Creamos un nuevo método constructor que no recibe conexión
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        Self {
            user_repository: Arc::new(
                UserRepositoryImpl::new().expect("Failed to create UserRepositoryImpl")
            ),
        }
    }
}

impl RepositoryRegistry for DatabaseRepositoryRegistry {
    fn user_repository(&self) -> &dyn UserRepositoryPort {
        // Aquí usamos as_ref() para obtener una referencia al contenido del Arc
        self.user_repository.as_ref()
    }
}

// Implementación concreta de UnitOfWork que usa DatabaseManager
pub struct DatabaseUnitOfWork {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl DatabaseUnitOfWork {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        Self { pool }
    }
    
    // Método para ejecutar operaciones en transacción
    pub async fn execute_create_user(&self, user: User) -> Result<User> {
        let mut conn = self.pool.get()?;
        
        conn.transaction(|_conn| {
            // En una implementación real, usaríamos _conn para las operaciones
            // Para simplificar, usamos el pool directamente
            let registry = DatabaseRepositoryRegistry::new(self.pool.clone());
            
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
            let registry = DatabaseRepositoryRegistry::new(self.pool.clone());
            let user_repo = registry.user_repository();
            
            let runtime = tokio::runtime::Handle::current();
            runtime.block_on(async {
                user_repo.update(user.clone()).await
            })
        })
    }
    
    // Puedes añadir más métodos para operaciones específicas según sea necesario
}