use crate::Infrastructure::config::AppConfig;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};
use diesel::PgConnection;
use lazy_static::lazy_static;
use log::{error, info, debug};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;
use anyhow::{Result, anyhow};

// Definición de tipos para mayor claridad
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

// Configuración para una base de datos
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub name: String,
    pub url: String,
    pub max_connections: u32,
    pub min_idle: Option<u32>,
    pub connection_timeout: u64,
    pub idle_timeout: Option<u64>,
}

impl DatabaseConfig {
    pub fn from_env(prefix: &str) -> Result<Self> {
        let name = prefix.to_string();
        let url_key = format!("{}_DATABASE_URL", prefix.to_uppercase());
        let max_conn_key = format!("{}_DATABASE_MAX_CONNECTIONS", prefix.to_uppercase());
        let min_idle_key = format!("{}_DATABASE_MIN_IDLE", prefix.to_uppercase());
        let timeout_key = format!("{}_DATABASE_CONNECTION_TIMEOUT", prefix.to_uppercase());
        let idle_timeout_key = format!("{}_DATABASE_IDLE_TIMEOUT", prefix.to_uppercase());
        
        let url = env::var(&url_key)
            .map_err(|_| anyhow!("{} must be set", url_key))?;
            
        let max_connections = env::var(&max_conn_key)
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .unwrap_or(10);
            
        let min_idle = env::var(&min_idle_key)
            .ok()
            .and_then(|v| v.parse::<u32>().ok());
            
        let connection_timeout = env::var(&timeout_key)
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .unwrap_or(30);
            
        let idle_timeout = env::var(&idle_timeout_key)
            .ok()
            .and_then(|v| v.parse::<u64>().ok());
            
        Ok(Self {
            name,
            url,
            max_connections,
            min_idle,
            connection_timeout,
            idle_timeout,
        })
    }
    
    pub fn new(name: &str, url: &str, max_connections: u32) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            max_connections,
            min_idle: None,
            connection_timeout: 30,
            idle_timeout: None,
        }
    }
}

// Estructura para gestionar múltiples pools de conexiones
pub struct DatabaseManager {
    pools: HashMap<String, DbPool>,
    default_db: Option<String>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
            default_db: None,
        }
    }

    // Registrar una nueva base de datos con su configuración
    pub fn register_database(&mut self, config: &DatabaseConfig) -> Result<(), PoolError> {
        info!("Registrando base de datos: {}", config.name);
        
        let manager = ConnectionManager::<PgConnection>::new(&config.url);
        let mut builder = Pool::builder()
            .max_size(config.max_connections)
            .connection_timeout(Duration::from_secs(config.connection_timeout));
            
        if let Some(idle) = config.min_idle {
            builder = builder.min_idle(Some(idle));
        }
        
        if let Some(idle_timeout) = config.idle_timeout {
            builder = builder.idle_timeout(Some(Duration::from_secs(idle_timeout)));
        }
        
        let pool = builder.build(manager)?;
        
        // Verificar que la conexión funciona
        let _conn = pool.get()?;
        info!("Conexión a base de datos {} establecida exitosamente", config.name);
        
        // Si es la primera base de datos, establecerla como default
        if self.pools.is_empty() && self.default_db.is_none() {
            self.default_db = Some(config.name.clone());
            info!("Estableciendo {} como base de datos predeterminada", config.name);
        }
        
        self.pools.insert(config.name.clone(), pool);
        Ok(())
    }
    
    // Establecer una base de datos predeterminada
    pub fn set_default_database(&mut self, name: &str) -> Result<(), String> {
        if self.pools.contains_key(name) {
            self.default_db = Some(name.to_string());
            Ok(())
        } else {
            Err(format!("Base de datos {} no encontrada", name))
        }
    }
    
    // Obtener el nombre de la base de datos predeterminada
    pub fn get_default_database(&self) -> Option<String> {
        self.default_db.clone()
    }
    
    // Obtener un pool de conexiones por nombre
    pub fn get_pool(&self, db_name: &str) -> Option<&DbPool> {
        self.pools.get(db_name)
    }

    
    // Obtener una conexión de un pool específico
    pub fn get_connection(&self, name: &str) -> Result<DbConnection, PoolError> {
        match self.get_pool(name) {
            Some(pool) => {
                debug!("Obteniendo conexión de pool: {}", name);
                pool.get()
            },
            None => {
                error!("Pool de conexiones no encontrado: {}", name);
                // Obtener cualquier error de PoolError
                let dummy_manager = ConnectionManager::<PgConnection>::new("");
                let dummy_pool = Pool::builder()
                    .max_size(1)
                    .build(dummy_manager)
                    .expect("Failed to create dummy pool");
                dummy_pool.get() // Esto siempre fallará con un error válido de PoolError
            }
        }
    }
    
    // Obtener una conexión del pool predeterminado
    pub fn get_default_connection(&self) -> Result<DbConnection, PoolError> {
        match &self.default_db {
            Some(name) => self.get_connection(name),
            None => {
                error!("No hay base de datos predeterminada configurada");
                // Obtener cualquier error de PoolError
                let dummy_manager = ConnectionManager::<PgConnection>::new("");
                let dummy_pool = Pool::builder()
                    .max_size(1)
                    .build(dummy_manager)
                    .expect("Failed to create dummy pool");
                dummy_pool.get() // Esto siempre fallará con un error válido de PoolError
            }
        }
    }
    
    // Verificar el estado de todas las conexiones
    pub fn check_health(&self) -> HashMap<String, bool> {
        let mut results = HashMap::new();
        
        for (name, pool) in &self.pools {
            let is_healthy = match pool.get() {
                Ok(_) => true,
                Err(e) => {
                    error!("Error en pool {}: {}", name, e);
                    false
                }
            };
            
            results.insert(name.clone(), is_healthy);
        }
        
        results
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
    
    // Base de datos principal (siempre debería existir)
    let main_config = DatabaseConfig::from_env("MAIN")
        .map_err(|e| {
            error!("Error al cargar configuración de base de datos principal: {}", e);
            e
        })?;
        
    manager.register_database(&main_config)?;
    
    // Bases de datos adicionales (configurables)
    let extra_dbs = env::var("EXTRA_DATABASES")
        .unwrap_or_else(|_| "".to_string());
        
    if !extra_dbs.is_empty() {
        for db_name in extra_dbs.split(',') {
            let db_name = db_name.trim();
            if !db_name.is_empty() {
                info!("Configurando base de datos adicional: {}", db_name);
                if let Ok(config) = DatabaseConfig::from_env(db_name) {
                    if let Err(e) = manager.register_database(&config) {
                        error!("Error al registrar base de datos {}: {}", db_name, e);
                    }
                } else {
                    error!("Error al cargar configuración para base de datos: {}", db_name);
                }
            }
        }
    }
    
    // Configurar base de datos predeterminada (si está especificada)
    if let Ok(default_db) = env::var("DEFAULT_DATABASE") {
        if let Err(e) = manager.set_default_database(&default_db) {
            error!("Error al configurar base de datos predeterminada: {}", e);
        }
    }
    
    Ok(())
}

// Función para obtener una conexión específica
pub fn get_connection(db_name: &str) -> Result<DbConnection, PoolError> {
    let manager = DB_MANAGER.lock().unwrap();
    manager.get_connection(db_name)
}

// Función para obtener la conexión predeterminada
pub fn get_default_connection() -> Result<DbConnection, PoolError> {
    let manager = DB_MANAGER.lock().unwrap();
    manager.get_default_connection()
}

// Función para obtener el nombre de la base de datos predeterminada
pub fn get_default_database_name() -> Option<String> {
    let manager = DB_MANAGER.lock().unwrap();
    manager.get_default_database()
}

// Función para verificar el estado de todas las conexiones
pub fn check_database_health() -> HashMap<String, bool> {
    let manager = DB_MANAGER.lock().unwrap();
    manager.check_health()
}
   
pub fn initialize_with_config(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    info!("Inicializando conexiones a bases de datos desde configuración");
    
    let mut manager = DB_MANAGER.lock().unwrap();
    
    // Base de datos principal
    manager.register_database(&config.main_db_config)?;
    
    // Base de datos analítica (si está configurada)
    if let Some(analytics_config) = &config.analytics_db_config {
        manager.register_database(analytics_config)?;
    }
    
    Ok(())
}

pub fn get_pool_from_connection() -> Pool<ConnectionManager<PgConnection>> {
    let manager = DB_MANAGER.lock().unwrap();
    match &manager.default_db {
        Some(name) => match manager.pools.get(name) {
            Some(pool) => pool.clone(),
            None => panic!("No hay pool de conexiones para la base de datos predeterminada")
        },
        None => panic!("No hay base de datos predeterminada configurada")
    }
}