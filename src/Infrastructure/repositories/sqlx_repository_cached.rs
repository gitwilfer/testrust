// src/Infrastructure/repositories/sqlx_repository_cached.rs
// *** ACTUALIZADO: Eliminados los ::new() ***

use std::sync::Arc;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};
use anyhow::{Result, anyhow, Context};
use tokio::sync::RwLock;
use sqlx::{Pool, Postgres, Transaction, Row};
use uuid::Uuid;

// Estructura para entradas de caché con TTL
#[derive(Clone)]
struct CacheEntry<V> {
    value: V,
    expiry: Instant,
}

// Repositorio base con caché
#[derive(Debug)] // Añadir Debug si es útil
pub struct SqlxCachedRepository<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pool: Arc<Pool<Postgres>>,
    entity_name: String,
    cache: RwLock<HashMap<K, CacheEntry<V>>>,
    cache_ttl: Duration,
}

impl<K, V> SqlxCachedRepository<K, V>
where
    K: Eq + Hash + Clone + Send + Sync, // Añadir Send + Sync a K
    V: Clone + Send + Sync, // Añadir Send + Sync a V
{
    /// Constructor Preferido: Recibe el pool (Inyección de Dependencias).
    pub fn with_pool(pool: Arc<Pool<Postgres>>, entity_name: &str, cache_ttl_seconds: u64) -> Self {
        Self {
            pool,
            entity_name: entity_name.to_string(),
            cache: RwLock::new(HashMap::new()),
            cache_ttl: Duration::from_secs(cache_ttl_seconds),
        }
    }

    // --- Métodos existentes sin cambios ---
    pub fn pool(&self) -> &Pool<Postgres> { &self.pool }
    pub fn entity_name(&self) -> &str { &self.entity_name }
    pub async fn get_from_cache(&self, key: &K) -> Option<V> { /* ... */ }
    pub async fn put_in_cache(&self, key: K, value: V) { /* ... */ }
    pub async fn invalidate_cache(&self, key: &K) { /* ... */ }
    pub async fn clean_cache(&self) { /* ... */ }
    pub async fn begin_transaction(&self) -> Result<Transaction<'_, Postgres>> { /* ... */ }
    pub async fn execute_in_transaction<F, R>(&self, operation: F) -> Result<R> where /* ... */ { /* ... */ }
}


#[derive(Debug)] // Añadir Debug
pub struct UserCachedRepository {
    inner: SqlxCachedRepository<Uuid, crate::Domain::entities::user::User>,
}

impl UserCachedRepository {

    /// Constructor Preferido: Recibe el pool (Inyección de Dependencias).
    pub fn with_pool(pool: Arc<Pool<Postgres>>) -> Self { // Quitado Result<>
         Self {
             inner: SqlxCachedRepository::with_pool(pool, "user", 300), // Llama al with_pool base
         }
    }

    // --- Métodos find_by_id, update, etc. sin cambios ---
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<crate::Domain::entities::user::User>> { /* ... */ }
    pub async fn update(&self, user: crate::Domain::entities::user::User) -> Result<crate::Domain::entities::user::User> { /* ... */ }
}
