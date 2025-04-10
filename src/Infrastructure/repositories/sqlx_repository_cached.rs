// src/Infrastructure/repositories/sqlx_repository_cached.rs

use std::sync::Arc;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use tokio::sync::RwLock;
use sqlx::{Pool, Postgres, Transaction};
use uuid::Uuid;

use crate::Infrastructure::Persistence::sqlx_database;
use crate::Application::services::get_database_for_entity;

// Estructura para un elemento en caché
struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

// Repositorio base con caché
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
    K: Eq + Hash + Clone,
    V: Clone,
{
    // Crear nuevo repositorio base para una entidad específica
    pub async fn new(entity_name: &str, cache_ttl_seconds: u64) -> Result<Self> {
        let db_name = get_database_for_entity(entity_name);
        let pool = sqlx_database::get_pool(&db_name).await?;
        
        Ok(Self {
            pool: Arc::new(pool),
            entity_name: entity_name.to_string(),
            cache: RwLock::new(HashMap::new()),
            cache_ttl: Duration::from_secs(cache_ttl_seconds),
        })
    }
    
    // Crear desde un pool existente
    pub fn with_pool(pool: Arc<Pool<Postgres>>, entity_name: &str, cache_ttl_seconds: u64) -> Self {
        Self {
            pool,
            entity_name: entity_name.to_string(),
            cache: RwLock::new(HashMap::new()),
            cache_ttl: Duration::from_secs(cache_ttl_seconds),
        }
    }
    
    // Obtener referencia al pool
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
    
    // Obtener el nombre de la entidad
    pub fn entity_name(&self) -> &str {
        &self.entity_name
    }
    
    // Buscar en caché
    pub async fn get_from_cache(&self, key: &K) -> Option<V> {
        let cache = self.cache.read().await;
        if let Some(entry) = cache.get(key) {
            if Instant::now() < entry.expires_at {
                return Some(entry.value.clone());
            }
        }
        None
    }
    
    // Guardar en caché
    pub async fn put_in_cache(&self, key: K, value: V) {
        let mut cache = self.cache.write().await;
        cache.insert(key, CacheEntry {
            value,
            expires_at: Instant::now() + self.cache_ttl,
        });
    }
    
    // Invalidar cache para una clave
    pub async fn invalidate_cache(&self, key: &K) {
        let mut cache = self.cache.write().await;
        cache.remove(key);
    }
    
    // Limpiar caché expirada
    pub async fn clean_cache(&self) {
        let mut cache = self.cache.write().await;
        let now = Instant::now();
        cache.retain(|_, entry| entry.expires_at > now);
    }
    
    // Iniciar una transacción
    pub async fn begin_transaction(&self) -> Result<Transaction<'_, Postgres>> {
        Ok(self.pool.begin().await?)
    }
    
    // Ejecutar operación en una transacción
    pub async fn execute_in_transaction<F, R>(&self, operation: F) -> Result<R>
    where
        F: for<'a> FnOnce(&'a mut Transaction<'_, Postgres>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R>> + Send + 'a>>,
        R: Send + 'static,
    {
        let mut tx = self.pool.begin().await?;
        let result = operation(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
    }
}

// Implementación específica para usuarios con caché por ID
pub struct UserCachedRepository {
    inner: SqlxCachedRepository<Uuid, crate::Domain::entities::user::User>,
}

impl UserCachedRepository {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            inner: SqlxCachedRepository::new("user", 300).await?, // Caché de 5 minutos
        })
    }
    
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<crate::Domain::entities::user::User>> {
        // Intentar primero desde caché
        if let Some(user) = self.inner.get_from_cache(&id).await {
            return Ok(Some(user));
        }
        
        // Si no está en caché, buscar en base de datos
        let user = sqlx::query!(
            r#"
            SELECT 
                idx_usuario as "id: Uuid",
                usuario as "username", 
                nombre as "first_name", 
                apellido as "last_name",
                correo_electronico as "email", 
                password_hash as "password",
                status as "status: i16",
                creado_por as "created_by: Option<Uuid>",
                fecha_creacion as "created_at: NaiveDateTime",
                modificado_por as "modified_by: Option<Uuid>",
                fecha_modificacion as "modified_at: Option<NaiveDateTime>"
            FROM usuarios 
            WHERE idx_usuario = $1
            "#,
            id
        )
        .fetch_optional(self.inner.pool())
        .await?;
        
        if let Some(r) = user {
            let user = crate::Domain::entities::user::User {
                id: r.id,
                username: r.username,
                first_name: r.first_name,
                last_name: r.last_name,
                email: r.email,
                password: r.password,
                status: r.status,
                created_by: r.created_by,
                created_at: r.created_at,
                modified_by: r.modified_by,
                modified_at: r.modified_at,
            };
            
            // Guardar en caché
            self.inner.put_in_cache(id, user.clone()).await;
            
            return Ok(Some(user));
        }
        
        Ok(None)
    }
    
    // Otras funciones como find_by_username, etc.
    // ...
    
    // Función para actualizar - debe invalidar caché
    pub async fn update(&self, user: crate::Domain::entities::user::User) -> Result<crate::Domain::entities::user::User> {
        // Invalidar caché antes de actualizar
        self.inner.invalidate_cache(&user.id).await;
        
        // Implementación de actualización
        // ...
        
        Ok(user)
    }
}