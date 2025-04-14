// src/Infrastructure/repositories/sqlx_batch_repository.rs

// use async_trait::async_trait;
use anyhow::{Result, anyhow};
use futures::{stream, StreamExt};
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;
use uuid::Uuid;

use crate::Domain::entities::user::User;
use crate::Infrastructure::repositories::sqlx_repository_base::SqlxRepositoryBase;

// Repositorio para operaciones en lote
pub struct BatchRepository {
    base: SqlxRepositoryBase,
    batch_size: usize,
}

impl BatchRepository {
    pub async fn new(entity_name: &str, batch_size: usize) -> Result<Self> {
        Ok(Self {
            base: SqlxRepositoryBase::new(entity_name).await?,
            batch_size,
        })
    }
    
    pub fn with_pool(pool: Arc<Pool<Postgres>>, entity_name: &str, batch_size: usize) -> Self {
        Self {
            base: SqlxRepositoryBase::with_pool(pool, entity_name),
            batch_size,
        }
    }
    
    // Insertar usuarios en lote
    pub async fn bulk_insert_users(&self, users: Vec<User>) -> Result<Vec<User>> {
        if users.is_empty() {
            return Ok(vec![]);
        }
        
        let chunks = users.chunks(self.batch_size);
        let mut results = Vec::with_capacity(users.len());
        
        for chunk in chunks {
            let inserted = self.insert_user_batch(chunk.to_vec()).await?;
            results.extend(inserted);
        }
        
        Ok(results)
    }
    
    // Insertar un lote de usuarios
    async fn insert_user_batch(&self, users: Vec<User>) -> Result<Vec<User>> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO usuarios (idx_usuario, usuario, nombre, apellido, correo_electronico, contraseña, estado, creado_por, fecha_creacion, modificado_por, fecha_modificacion) "
        );
        
        query_builder.push_values(users.iter(), |mut b, user| {
            b.push_bind(user.id)
             .push_bind(&user.username)
             .push_bind(&user.first_name)
             .push_bind(&user.last_name)
             .push_bind(&user.email)
             .push_bind(&user.password)
             .push_bind(user.status)
             .push_bind(user.created_by)
             .push_bind(user.created_at)
             .push_bind(user.modified_by)
             .push_bind(user.modified_at);
        });
        
        query_builder.push(" RETURNING idx_usuario");
        
        let query = query_builder.build();
        
        // Corrección: usar try_get para acceder a los datos de PgRow
        let result = query.fetch_all(self.base.pool()).await;
        let inserted_ids: Vec<Uuid> = match result {
            Ok(rows) => {
                // Convertir los resultados al tipo esperado
                let mut ids = Vec::with_capacity(rows.len());
                for row in rows {
                    // Usar la implementación correcta de Row::try_get para Postgres
                    match row.try_get::<Uuid, _>("idx_usuario") {
                        Ok(id) => ids.push(id),
                        Err(_) => {
                            // Intentar con índice si el nombre de columna no funciona
                            match row.try_get::<Uuid, _>(0) {
                                Ok(id) => ids.push(id),
                                Err(e) => return Err(anyhow!("Error al extraer UUID de la fila: {}", e)),
                            }
                        }
                    }
                }
                ids
            },
            Err(e) => return Err(anyhow!("Error al insertar usuarios en lote: {}", e)),
        };
            
        // Mapear los IDs insertados de vuelta a los usuarios originales
        let id_map: std::collections::HashMap<Uuid, User> = users.into_iter()
            .map(|user| (user.id, user))
            .collect();
            
        let results = inserted_ids.into_iter()
            .filter_map(|id| id_map.get(&id).cloned())
            .collect();
            
        Ok(results)
    }
    
    // Actualizar usuarios en lote con procesamiento paralelo
    pub async fn bulk_update_users(&self, users: Vec<User>, concurrency: usize) -> Result<Vec<User>> {
        if users.is_empty() {
            return Ok(vec![]);
        }
        
        // Actualizar en paralelo con límite de concurrencia
        let results = stream::iter(users)
            .map(|user| {
                let pool = self.base.pool().clone();
                async move {
                    // Reemplazamos el macro query! con query normal
                    let sql = r#"
                        UPDATE usuarios
                        SET 
                            usuario = $1,
                            nombre = $2, 
                            apellido = $3, 
                            correo_electronico = $4,
                            contraseña = $5,
                            estado = $6,
                            modificado_por = $7,
                            fecha_modificacion = $8
                        WHERE idx_usuario = $9
                        RETURNING idx_usuario
                    "#;
                    
                    let result = sqlx::query(sql)
                        .bind(&user.username)
                        .bind(&user.first_name)
                        .bind(&user.last_name)
                        .bind(&user.email)
                        .bind(&user.password)
                        .bind(user.status)
                        .bind(user.modified_by)
                        .bind(user.modified_at)
                        .bind(user.id)
                        .execute(&pool)
                        .await;
                    
                    match result {
                        Ok(_) => Ok(user),
                        Err(e) => Err(anyhow!("Error actualizando usuario {}: {}", user.id, e)),
                    }
                }
            })
            .buffer_unordered(concurrency)
            .collect::<Vec<Result<User>>>()
            .await;
            
        // Procesar resultados
        let mut successful = Vec::new();
        let mut errors = Vec::new();
        
        for result in results {
            match result {
                Ok(user) => successful.push(user),
                Err(e) => errors.push(e),
            }
        }
        
        if !errors.is_empty() {
            return Err(anyhow!("Errores en la actualización masiva: {:?}", errors));
        }
        
        Ok(successful)
    }
}