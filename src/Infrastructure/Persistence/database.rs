use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};
use diesel::PgConnection;
use lazy_static::lazy_static;
use log::{error, info};
use std::env;
use std::sync::Arc;
use std::time::Duration;

// Tipo para el pool de conexiones
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

// Estructura para gestionar múltiples pools de conexiones
pub struct DatabaseManager {
    pools: std::collections::HashMap<String, DbPool>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self {
            pools: std::collections::HashMap::new(),
        }
    }

    // Registrar una nueva base de datos con su configuración
    pub fn register_database(&mut self, name: &str, url: &str, max_size: u32, min_idle: Option<u32>) -> Result<(), PoolError> {
        info!("Registrando base de datos: {}", name);
        
        let manager = ConnectionManager::<PgConnection>::new(url);
        let mut builder = Pool::builder()
            .max_size(max_size)
            .connection_timeout(Duration::from_secs(30));
            
        if let Some(idle) = min_idle {
            builder = builder.min_idle(Some(idle));
        }
        
        let pool = builder.build(manager)?;
        
        // Verificar que la conexión funciona
        let _conn = pool.get()?;
        info!("Conexión a base de datos {} establecida exitosamente", name);
        
        self.pools.insert(name.to_string(), pool);
        Ok(())
    }
    
    // Obtener un pool de conexiones por nombre
    pub fn get_pool(&self, name: &str) -> Option<&DbPool> {
        self.pools.get(name)
    }
    
    // Obtener una conexión de un pool específico
    pub fn get_connection(&self, name: &str) -> Result<DbConnection, PoolError> {
        match self.get_pool(name) {
            Some(pool) => {
                let conn = pool.get()?;
                Ok(conn)
            },
            None => {
                error!("Pool de conexiones no encontrado: {}", name);
                Err(PoolError::ConnectionError(r2d2::Error::ConnectionError("Pool not found".into())))
            }
        }
    }
}

// Singleton para acceso global al gestor de bases de datos
lazy_static! {
    static ref DB_MANAGER: Arc<std::sync::Mutex<DatabaseManager>> = Arc::new(std::sync::Mutex::new(
        DatabaseManager::new()
    ));
}

// Inicializar todas las conexiones a bases de datos desde la configuración
pub fn initialize_databases() -> Result<(), Box<dyn std::error::Error>> {
    info!("Inicializando conexiones a bases de datos");
    
    let mut manager = DB_MANAGER.lock().unwrap();
    
    // Base de datos principal
    let main_db_url = env::var("MAIN_DATABASE_URL").expect("MAIN_DATABASE_URL debe estar configurada");
    let max_connections = env::var("MAIN_DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "10".to_string())
        .parse::<u32>()
        .unwrap_or(10);
    let min_idle = env::var("DATABASE_MIN_IDLE")
        .ok()
        .and_then(|v| v.parse::<u32>().ok());
    
    manager.register_database("main", &main_db_url, max_connections, min_idle)?;
    
    // Aquí puedes registrar bases de datos adicionales
    // Por ejemplo:
    if let Ok(analytics_url) = env::var("ANALYTICS_DATABASE_URL") {
        let analytics_max = env::var("ANALYTICS_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u32>()
            .unwrap_or(5);
        manager.register_database("analytics", &analytics_url, analytics_max, None)?;
    }
    
    Ok(())
}

// Función para obtener una conexión
pub fn get_connection(db_name: &str) -> Result<DbConnection, PoolError> {
    let manager = DB_MANAGER.lock().unwrap();
    manager.get_connection(db_name)
}

// Función para obtener la conexión principal (conveniencia)
pub fn get_main_connection() -> Result<DbConnection, PoolError> {
    get_connection("main")
}
