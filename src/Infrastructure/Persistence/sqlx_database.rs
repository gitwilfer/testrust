// src/Infrastructure/Persistence/sqlx_database.rs

use anyhow::{Result, anyhow, Context};
use lazy_static::lazy_static;
use log::{error, info};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::Infrastructure::config::AppConfig;

// Tipo para mayor claridad
pub type SqlxPool = Pool<Postgres>;

// Configuración para una base de datos SQLx
#[derive(Debug, Clone)]
pub struct SqlxDatabaseConfig {
    pub name: String,
    pub url: String,
    pub max_connections: u32,
    pub min_connections: Option<u32>,
    pub acquire_timeout: u64,
    pub idle_timeout: Option<u64>,
    pub max_lifetime: Option<u64>,
}

impl SqlxDatabaseConfig {
    pub fn from_env(prefix: &str) -> Result<Self> {
        let name = prefix.to_string();
        let url_key = format!("{}_DATABASE_URL", prefix.to_uppercase());
        let max_conn_key = format!("{}_DATABASE_MAX_CONNECTIONS", prefix.to_uppercase());
        let min_conn_key = format!("{}_DATABASE_MIN_CONNECTIONS", prefix.to_uppercase());
        let timeout_key = format!("{}_DATABASE_ACQUIRE_TIMEOUT", prefix.to_uppercase());
        let idle_timeout_key = format!("{}_DATABASE_IDLE_TIMEOUT", prefix.to_uppercase());
        let lifetime_key = format!("{}_DATABASE_MAX_LIFETIME", prefix.to_uppercase());
        
        let url = env::var(&url_key)
            .map_err(|_| anyhow!("{} must be set", url_key))?;
            
        let max_connections = env::var(&max_conn_key)
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .unwrap_or(10);
            
        let min_connections = env::var(&min_conn_key)
            .ok()
            .and_then(|v| v.parse::<u32>().ok());
            
        let acquire_timeout = env::var(&timeout_key)
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .unwrap_or(30);
            
        let idle_timeout = env::var(&idle_timeout_key)
            .ok()
            .and_then(|v| v.parse::<u64>().ok());
            
        let max_lifetime = env::var(&lifetime_key)
            .ok()
            .and_then(|v| v.parse::<u64>().ok());
            
        Ok(Self {
            name,
            url,
            max_connections,
            min_connections,
            acquire_timeout,
            idle_timeout,
            max_lifetime,
        })
    }
    
    pub fn from_diesel_config(diesel_config: &crate::Infrastructure::Persistence::database::DatabaseConfig) -> Self {
        Self {
            name: diesel_config.name.clone(),
            url: diesel_config.url.clone(),
            max_connections: diesel_config.max_connections,
            min_connections: diesel_config.min_idle,
            acquire_timeout: diesel_config.connection_timeout,
            idle_timeout: diesel_config.idle_timeout,
            max_lifetime: None,
        }
    }
}

// Gestor de pools de SQLx
pub struct SqlxDatabaseManager {
    pools: HashMap<String, SqlxPool>,
    default_db: Option<String>,
}

impl SqlxDatabaseManager {
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
            default_db: None,
        }
    }

    // Registrar una nueva base de datos con su configuración
    pub async fn register_database(&mut self, config: &SqlxDatabaseConfig) -> Result<()> {
        info!("Registrando base de datos SQLx: {}", config.name);
        
        let mut pool_options = PgPoolOptions::new()
            .max_connections(config.max_connections);
            
        if let Some(min_conn) = config.min_connections {
            pool_options = pool_options.min_connections(min_conn);
        }
        
        pool_options = pool_options.acquire_timeout(Duration::from_secs(config.acquire_timeout));
        
        if let Some(idle_timeout) = config.idle_timeout {
            pool_options = pool_options.idle_timeout(Duration::from_secs(idle_timeout));
        }
        
        if let Some(max_lifetime) = config.max_lifetime {
            pool_options = pool_options.max_lifetime(Duration::from_secs(max_lifetime));
        }
        
        let pool = pool_options
            .connect(&config.url)
            .await
            .with_context(|| format!("Failed to connect to database {}", config.name))?;
        
        // Verificar que la conexión funciona
        let _ = pool.acquire().await?;
        info!("Conexión a base de datos SQLx {} establecida exitosamente", config.name);
        
        // Si es la primera base de datos, establecerla como default
        if self.pools.is_empty() && self.default_db.is_none() {
            self.default_db = Some(config.name.clone());
            info!("Estableciendo {} como base de datos SQLx predeterminada", config.name);
        }
        
        self.pools.insert(config.name.clone(), pool);
        Ok(())
    }
    
    // Establecer una base de datos predeterminada
    pub fn set_default_database(&mut self, name: &str) -> Result<()> {
        if self.pools.contains_key(name) {
            self.default_db = Some(name.to_string());
            Ok(())
        } else {
            Err(anyhow!("Base de datos SQLx {} no encontrada", name))
        }
    }
    
    // Obtener el nombre de la base de datos predeterminada
    pub fn get_default_database(&self) -> Option<String> {
        self.default_db.clone()
    }
    
    // Obtener un pool por nombre
    pub fn get_pool(&self, db_name: &str) -> Option<SqlxPool> {
        self.pools.get(db_name).cloned()
    }

    // Obtener el pool predeterminado
    pub fn get_default_pool(&self) -> Result<SqlxPool> {
        match &self.default_db {
            Some(name) => self.get_pool(name)
                .ok_or_else(|| anyhow!("Pool de conexiones SQLx no encontrado: {}", name)),
            None => Err(anyhow!("No hay base de datos SQLx predeterminada configurada"))
        }
    }
    
    // Verificar el estado de todas las conexiones
    pub async fn check_health(&self) -> HashMap<String, bool> {
        let mut results = HashMap::new();
        
        for (name, pool) in &self.pools {
            let is_healthy = match pool.acquire().await {
                Ok(_) => true,
                Err(e) => {
                    error!("Error en pool SQLx {}: {}", name, e);
                    false
                }
            };
            
            results.insert(name.clone(), is_healthy);
        }
        
        results
    }
}

// Singleton para acceso global al gestor de bases de datos SQLx
lazy_static! {
    static ref SQLX_DB_MANAGER: Arc<Mutex<SqlxDatabaseManager>> = Arc::new(Mutex::new(
        SqlxDatabaseManager::new()
    ));
}

// Inicializar todos los pools SQLx desde la configuración
pub async fn initialize_sqlx_databases() -> Result<()> {
    info!("Inicializando conexiones SQLx a bases de datos");
    
    let mut manager = SQLX_DB_MANAGER.lock()
        .map_err(|_| anyhow!("Error al obtener lock del gestor de bases de datos SQLx"))?;
    
    // Base de datos principal (siempre debería existir)
    let main_config = SqlxDatabaseConfig::from_env("MAIN")
        .map_err(|e| {
            error!("Error al cargar configuración SQLx de base de datos principal: {}", e);
            e
        })?;
        
    manager.register_database(&main_config).await?;
    
    // Bases de datos adicionales (configurables)
    let extra_dbs = env::var("EXTRA_DATABASES")
        .unwrap_or_else(|_| "".to_string());
        
    if !extra_dbs.is_empty() {
        for db_name in extra_dbs.split(',') {
            let db_name = db_name.trim();
            if !db_name.is_empty() {
                info!("Configurando base de datos SQLx adicional: {}", db_name);
                if let Ok(config) = SqlxDatabaseConfig::from_env(db_name) {
                    if let Err(e) = manager.register_database(&config).await {
                        error!("Error al registrar base de datos SQLx {}: {}", db_name, e);
                    }
                } else {
                    error!("Error al cargar configuración SQLx para base de datos: {}", db_name);
                }
            }
        }
    }
    
    // Configurar base de datos predeterminada (si está especificada)
    if let Ok(default_db) = env::var("DEFAULT_DATABASE") {
        if let Err(e) = manager.set_default_database(&default_db) {
            error!("Error al configurar base de datos SQLx predeterminada: {}", e);
        }
    }
    
    Ok(())
}

// Inicializar con la configuración existente de AppConfig
pub async fn initialize_with_config(config: &AppConfig) -> Result<()> {
    info!("Inicializando conexiones SQLx a bases de datos desde configuración");
    
    let mut manager = SQLX_DB_MANAGER.lock()
        .map_err(|_| anyhow!("Error al obtener lock del gestor de bases de datos SQLx"))?;
    
    // Convertir configuración Diesel a SQLx
    let main_sqlx_config = SqlxDatabaseConfig::from_diesel_config(&config.main_db_config);
    
    // Base de datos principal
    manager.register_database(&main_sqlx_config).await?;
    
    // Base de datos analítica (si está configurada)
    if let Some(analytics_config) = &config.analytics_db_config {
        let analytics_sqlx_config = SqlxDatabaseConfig::from_diesel_config(analytics_config);
        manager.register_database(&analytics_sqlx_config).await?;
    }
    
    Ok(())
}

// Función para obtener un pool específico
pub async fn get_pool(db_name: &str) -> Result<SqlxPool> {
    let manager = SQLX_DB_MANAGER.lock()
        .map_err(|_| anyhow!("Error al obtener lock del gestor de bases de datos SQLx"))?;
    
    manager.get_pool(db_name)
        .ok_or_else(|| anyhow!("Pool de conexiones SQLx no encontrado: {}", db_name))
}

// Función para obtener el pool predeterminado
pub async fn get_default_pool() -> Result<SqlxPool> {
    let manager = SQLX_DB_MANAGER.lock()
        .map_err(|_| anyhow!("Error al obtener lock del gestor de bases de datos SQLx"))?;
    
    manager.get_default_pool()
}

// Función para obtener el nombre de la base de datos predeterminada
pub fn get_default_database_name() -> Option<String> {
    if let Ok(manager) = SQLX_DB_MANAGER.lock() {
        manager.get_default_database()
    } else {
        None
    }
}

pub async fn check_database_health() -> Result<HashMap<String, bool>> {
    // Obtenemos el HashMap de pools primero, liberando el lock inmediatamente
    let pools_clone = {
        let manager = SQLX_DB_MANAGER.lock()
            .map_err(|_| anyhow!("Error al obtener lock del gestor de bases de datos SQLx"))?;
        
        // Clonar solo los nombres y pools para no mantener el MutexGuard
        let mut pools_map = HashMap::new();
        for (name, pool) in &manager.pools {
            pools_map.insert(name.clone(), pool.clone());
        }
        pools_map
    }; // El MutexGuard se libera aquí al finalizar el bloque
    
    // Ahora verificamos la salud de cada pool con el HashMap clonado
    let mut results = HashMap::new();
    
    for (name, pool) in pools_clone {
        let is_healthy = match pool.acquire().await {
            Ok(_) => true,
            Err(e) => {
                error!("Error en pool SQLx {}: {}", name, e);
                false
            }
        };
        
        results.insert(name, is_healthy);
    }
    
    Ok(results)
}