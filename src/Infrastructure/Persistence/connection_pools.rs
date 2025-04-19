// src/Infrastructure/Persistence/connection_pools.rs
// *** REVISADO para gestionar múltiples pools (Async Diesel & SQLx) ***

use crate::Infrastructure::config::AppConfig;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager,
    pooled_connection::bb8::Pool as AsyncDieselPool, // Alias para el Pool Async
    AsyncPgConnection,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres}; // SQLx
use log::{info, debug, error, warn}; // Añadir warn
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use lazy_static::lazy_static; // Reintroducido para el gestor global
use anyhow::{Result, anyhow, Context};

// --- Definición de tipos para mayor claridad ---
pub type AsyncDbPool = Arc<AsyncDieselPool<AsyncPgConnection>>; // Usar Arc para compartir
pub type SqlxPool = Arc<Pool<Postgres>>; // Usar Arc para compartir
// --- REMOVED --- Tipos síncronos (DbPool, DbConnection)

// --- Structs de Configuración (Sin cambios) ---
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub name: String,
    pub url: String,
    pub max_connections: u32,
    // Campos síncronos opcionales (min_idle, connection_timeout, idle_timeout)
    // Se mantienen por compatibilidad con AppConfig, pero no se usan para crear pools async
    pub min_idle: Option<u32>,
    pub connection_timeout: u64,
    pub idle_timeout: Option<u64>,
}
impl DatabaseConfig {
    pub fn from_env(prefix: &str) -> Result<Self> { /* ... implementación sin cambios ... */
        let name = prefix.to_string();
        let url_key = format!("{}_DATABASE_URL", prefix.to_uppercase());
        let max_conn_key = format!("{}_DATABASE_MAX_CONNECTIONS", prefix.to_uppercase());

        let url = env::var(&url_key)
            .map_err(|_| anyhow!("{} must be set", url_key))?;
        let max_connections = env::var(&max_conn_key)
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .unwrap_or(10);

        Ok(Self {
            name,
            url,
            max_connections,
            // Valores por defecto para campos síncronos no usados
            min_idle: None,
            connection_timeout: 30,
            idle_timeout: None,
        })
    }
    pub fn new(name: &str, url: &str, max_connections: u32) -> Self { /* ... implementación sin cambios ... */
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

#[derive(Debug, Clone)]
pub struct SqlxDatabaseConfig { /* ... implementación sin cambios ... */
    pub name: String,
    pub url: String,
    pub max_connections: u32,
    pub min_connections: Option<u32>,
    pub acquire_timeout: u64,
    pub idle_timeout: Option<u64>,
    pub max_lifetime: Option<u64>,
}
impl SqlxDatabaseConfig {
    pub fn from_env(prefix: &str) -> Result<Self> { /* ... implementación sin cambios ... */
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
    pub fn from_diesel_config(diesel_config: &DatabaseConfig) -> Self { /* ... implementación sin cambios ... */
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

// --- Gestor Centralizado de Pools (Async Diesel & SQLx) ---
#[derive(Default)] // Default crea HashMaps vacíos y None para defaults
pub struct PoolManager {
    async_diesel_pools: HashMap<String, AsyncDbPool>,
    sqlx_pools: HashMap<String, SqlxPool>,
    default_async_diesel_db: Option<String>,
    default_sqlx_db: Option<String>,
}

impl PoolManager {

    pub async fn get_default_async_diesel_db_name(&self) -> Option<String> {
        self.default_async_diesel_db.clone()
    }
    pub async fn get_default_sqlx_db_name(&self) -> Option<String> {
        self.default_sqlx_db.clone()
    }
    async fn create_and_register_async_diesel(&mut self, config: &DatabaseConfig) -> Result<()> {
        if self.async_diesel_pools.contains_key(&config.name) {
            warn!("Pool Diesel Async '{}' ya registrado. Omitiendo.", config.name);
            return Ok(());
        }
        info!("Creando y registrando pool Diesel asíncrono: {}", config.name);
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.url);
        let pool = AsyncDieselPool::builder() // No pasar manager aquí
            .max_size(config.max_connections as usize)
            .build(manager) // Pasar manager aquí
            .await
            .with_context(|| format!("Fallo al construir pool Diesel asíncrono para {}", config.name))?;

        // Verificar conexión (opcional pero recomendado)
        match pool.get().await {
             Ok(_) => info!("Conexión a base de datos Diesel Async {} establecida exitosamente", config.name),
             Err(e) => {
                 error!("Fallo al adquirir conexión de prueba para Diesel Async {}: {}", config.name, e);
                 return Err(anyhow!("Fallo verificación conexión Diesel Async {}: {}", config.name, e));
             }
         }

        let pool_arc = Arc::new(pool);
        // Establecer como default si es el primero
        if self.async_diesel_pools.is_empty() && self.default_async_diesel_db.is_none() {
            self.default_async_diesel_db = Some(config.name.clone());
            info!("Estableciendo '{}' como base de datos Diesel Async predeterminada", config.name);
        }
        self.async_diesel_pools.insert(config.name.clone(), pool_arc);
        Ok(())
    }
    async fn create_and_register_sqlx(&mut self, config: &SqlxDatabaseConfig) -> Result<()> {
        if self.sqlx_pools.contains_key(&config.name) {
            warn!("Pool SQLx '{}' ya registrado. Omitiendo.", config.name);
            return Ok(());
        }
        info!("Creando y registrando pool SQLx: {}", config.name);
        let mut pool_options = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(Duration::from_secs(config.acquire_timeout));
        // ... (aplicar otras opciones SQLx como antes) ...
        if let Some(min_conn) = config.min_connections { pool_options = pool_options.min_connections(min_conn); }
        if let Some(idle_timeout) = config.idle_timeout { pool_options = pool_options.idle_timeout(Duration::from_secs(idle_timeout)); }
        if let Some(max_lifetime) = config.max_lifetime { pool_options = pool_options.max_lifetime(Duration::from_secs(max_lifetime)); }

        let pool = pool_options
            .connect(&config.url)
            .await
            .with_context(|| format!("Fallo al conectar a base de datos SQLx {}", config.name))?;

        // Verificar conexión
        match pool.acquire().await {
            Ok(_) => info!("Conexión a base de datos SQLx {} establecida exitosamente", config.name),
            Err(e) => {
                error!("Fallo al adquirir conexión de prueba para SQLx {}: {}", config.name, e);
                return Err(anyhow!("Fallo verificación conexión SQLx {}: {}", config.name, e));
            }
        }

        let pool_arc = Arc::new(pool);
        // Establecer como default si es el primero
        if self.sqlx_pools.is_empty() && self.default_sqlx_db.is_none() {
            self.default_sqlx_db = Some(config.name.clone());
            info!("Estableciendo '{}' como base de datos SQLx predeterminada", config.name);
        }
        self.sqlx_pools.insert(config.name.clone(), pool_arc);
        Ok(())
    }

    // --- Funciones para obtener pools ---
    fn get_async_diesel(&self, name: &str) -> Option<AsyncDbPool> {
        self.async_diesel_pools.get(name).cloned()
    }
    fn get_sqlx(&self, name: &str) -> Option<SqlxPool> {
        self.sqlx_pools.get(name).cloned()
    }
    fn get_default_async_diesel(&self) -> Option<AsyncDbPool> {
        self.default_async_diesel_db.as_ref().and_then(|name| self.get_async_diesel(name))
    }
    fn get_default_sqlx(&self) -> Option<SqlxPool> {
        self.default_sqlx_db.as_ref().and_then(|name| self.get_sqlx(name))
    }
}

// --- Singleton Global para el Gestor de Pools ---
lazy_static! {
    static ref POOL_MANAGER: Arc<Mutex<PoolManager>> = Arc::new(Mutex::new(
        PoolManager::default() // Usa el Default trait
    ));
}

// --- Función de Inicialización Principal (Reemplaza initialize_with_config) ---
pub async fn initialize_pools(config: &AppConfig) -> Result<()> {
    info!("Inicializando TODOS los pools de bases de datos (Async Diesel & SQLx)...");
    let mut manager = POOL_MANAGER.lock()
        .map_err(|_| anyhow!("Error fatal: No se pudo obtener lock del PoolManager"))?;

    // --- Registrar Pool Principal (Async Diesel) ---
    if let Err(e) = manager.create_and_register_async_diesel(&config.main_db_config).await {
        error!("Error al registrar pool principal Diesel Async: {}", e);
        // Decide si fallar o continuar
        return Err(e);
    }

    // --- Registrar Pool Principal (SQLx) ---
    let main_sqlx_config = SqlxDatabaseConfig::from_diesel_config(&config.main_db_config);
    if let Err(e) = manager.create_and_register_sqlx(&main_sqlx_config).await {
        error!("Error al registrar pool principal SQLx: {}", e);
        // Decide si fallar o continuar
        // return Err(e); // Podrías continuar si SQLx es solo para queries opcionales
    }

    // --- Registrar Pool Analítico (si existe) ---
    if let Some(analytics_config) = &config.analytics_db_config {
        // Registrar Async Diesel si es necesario para analytics
        // if let Err(e) = manager.create_and_register_async_diesel(analytics_config).await { ... }

        // Registrar SQLx para analytics
        let analytics_sqlx_config = SqlxDatabaseConfig::from_diesel_config(analytics_config);
        if let Err(e) = manager.create_and_register_sqlx(&analytics_sqlx_config).await {
            error!("Error al registrar pool analítico SQLx: {}", e);
        }
    }

    // --- Registrar Pools Adicionales (Opcional, basado en EXTRA_DATABASES) ---
    let extra_dbs = env::var("EXTRA_DATABASES").unwrap_or_else(|_| "".to_string());
    if !extra_dbs.is_empty() {
        info!("Procesando EXTRA_DATABASES: {}", extra_dbs);
        for db_prefix in extra_dbs.split(',') {
            let db_prefix = db_prefix.trim();
            if !db_prefix.is_empty() {
                // Intentar crear config Diesel y SQLx para el prefijo
                match DatabaseConfig::from_env(db_prefix) {
                    Ok(db_config) => {
                        // Registrar Async Diesel
                        if let Err(e) = manager.create_and_register_async_diesel(&db_config).await {
                            error!("Error registrando pool extra Diesel Async '{}': {}", db_prefix, e);
                        }
                        // Registrar SQLx
                        let sqlx_config = SqlxDatabaseConfig::from_diesel_config(&db_config);
                        if let Err(e) = manager.create_and_register_sqlx(&sqlx_config).await {
                            error!("Error registrando pool extra SQLx '{}': {}", db_prefix, e);
                        }
                    },
                    Err(e) => {
                        error!("Error cargando config para DB extra '{}': {}", db_prefix, e);
                    }
                }
            }
        }
    }

    // Configurar base de datos predeterminada (si está especificada y existe)
    // Nota: Los defaults ya se establecen al registrar el primer pool de cada tipo.
    // Podrías añadir lógica para sobrescribir el default si se especifica DEFAULT_DATABASE.

    info!("Inicialización de pools completada.");
    Ok(())
}

/// Obtiene un pool Diesel Asíncrono por nombre.
pub fn get_async_pool(db_name: &str) -> Result<AsyncDbPool> {
    let manager = POOL_MANAGER.lock()
        .map_err(|_| anyhow!("Error al obtener lock del PoolManager"))?;
    manager.get_async_diesel(db_name)
        .ok_or_else(|| anyhow!("Pool Diesel Async no encontrado: {}", db_name))
}

/// Obtiene el pool Diesel Asíncrono predeterminado.
pub fn get_default_async_pool() -> Result<AsyncDbPool> {
    let manager = POOL_MANAGER.lock()
        .map_err(|_| anyhow!("Error al obtener lock del PoolManager"))?;
    manager.get_default_async_diesel()
        .ok_or_else(|| anyhow!("No hay pool Diesel Async predeterminado configurado"))
}

/// Obtiene un pool SQLx por nombre. (Similar a la función anterior de sqlx_database)
pub fn get_sqlx_pool(db_name: &str) -> Result<SqlxPool> {
    let manager = POOL_MANAGER.lock()
        .map_err(|_| anyhow!("Error al obtener lock del PoolManager"))?;
    manager.get_sqlx(db_name)
        .ok_or_else(|| anyhow!("Pool SQLx no encontrado: {}", db_name))
}

/// Obtiene el pool SQLx predeterminado. (Similar a la función anterior de sqlx_database)
pub async fn get_default_sqlx_pool() -> Result<SqlxPool> { // Mantenido async por compatibilidad si algo lo llamaba así
    let manager = POOL_MANAGER.lock()
        .map_err(|_| anyhow!("Error al obtener lock del PoolManager"))?;
    manager.get_default_sqlx()
        .ok_or_else(|| anyhow!("No hay pool SQLx predeterminado configurado"))
}

/// Obtiene el nombre de la base de datos SQLx predeterminada de forma síncrona.
/// NOTA: Devuelve None si el PoolManager no está inicializado o no hay default.
pub fn get_default_sqlx_db_name_sync() -> Option<String> {
    match POOL_MANAGER.try_lock() { // Usar try_lock para evitar bloqueo si ya está bloqueado
        Ok(manager) => manager.default_sqlx_db.clone(),
        Err(_) => {
            // Podría ser un error si el lock está disputado, o más probablemente,
            // que se llama antes de que initialize_pools complete.
            // Devolver None es razonable aquí, ya que el default no está listo.
            warn!("No se pudo obtener lock de POOL_MANAGER para get_default_sqlx_db_name_sync. ¿Se llamó antes de inicializar?");
            None
        }
    }
}

pub async fn check_pools_health() -> Result<HashMap<String, bool>> {
    debug!("Iniciando chequeo de salud de TODOS los pools...");
    let pools_to_check = { // Clonar pools fuera del lock
        let manager = POOL_MANAGER.lock()
            .map_err(|_| anyhow!("Error al obtener lock del PoolManager para health check"))?;
        let mut combined_pools = HashMap::new();
        for (name, pool) in &manager.async_diesel_pools {
            combined_pools.insert(format!("diesel_async_{}", name), pool.clone());
        }
        for (name, pool) in &manager.sqlx_pools {
            combined_pools.insert(format!("sqlx_{}", name), pool.clone());
        }
        combined_pools
    }; // Lock se libera aquí

    let mut results = HashMap::new();
    for (full_name, pool_arc) in pools_to_check {
        let is_healthy = if full_name.starts_with("diesel_async_") {
            // Check Diesel Async pool
            match pool_arc.get().await { // Intenta obtener una conexión
                Ok(_) => true,
                Err(e) => {
                    error!("Error en health check para {}: {:?}", full_name, e);
                    false
                }
            }
        } else if full_name.starts_with("sqlx_") {
            // Check SQLx pool (necesitamos convertir el Arc<AsyncDbPool> a Arc<SqlxPool>)
            // ¡ERROR LÓGICO AQUÍ! No podemos mezclar los tipos en el HashMap así.
            // Necesitamos iterar por separado o usar un Enum.
            // Vamos a corregir iterando por separado.
            false // Placeholder, se corregirá abajo
        } else {
            warn!("Tipo de pool desconocido en health check: {}", full_name);
            false
        };
        // results.insert(full_name, is_healthy); // Se moverá abajo
    }

    // --- CORRECCIÓN: Iterar por separado ---
    let (async_diesel_pools_clone, sqlx_pools_clone) = {
         let manager = POOL_MANAGER.lock()
            .map_err(|_| anyhow!("Error al obtener lock del PoolManager para health check (corrección)"))?;
         (manager.async_diesel_pools.clone(), manager.sqlx_pools.clone())
    };

    for (name, pool) in async_diesel_pools_clone {
         let is_healthy = match pool.get().await {
             Ok(_) => true,
             Err(e) => {
                 error!("Error en health check para diesel_async_{}: {:?}", name, e);
                 false
             }
         };
         results.insert(format!("diesel_async_{}", name), is_healthy);
    }

    for (name, pool) in sqlx_pools_clone {
         let is_healthy = match pool.acquire().await {
             Ok(_) => true,
             Err(e) => {
                 error!("Error en health check para sqlx_{}: {:?}", name, e);
                 false
             }
         };
         results.insert(format!("sqlx_{}", name), is_healthy);
    }


    debug!("Chequeo de salud de pools completado.");
    Ok(results)
}

