use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use uuid::Uuid;

use crate::Application::ports::repositories::UserQueryRepository;
use crate::Domain::entities::user::User;
use crate::Infrastructure::repositories::sqlx_repository_base::SqlxRepositoryBase;

pub struct UserQueryRepositorySqlx {
    base: SqlxRepositoryBase,
    current_database: String,
}

impl UserQueryRepositorySqlx {
    pub async fn new() -> Result<Self> {
        let base = SqlxRepositoryBase::new("user").await?;
        
        Ok(Self {
            current_database: "main".to_string(),
            base,
        })
    }
    
    pub fn with_pool(pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            current_database: "main".to_string(),
            base: SqlxRepositoryBase::with_pool(pool, "user"),
        }
    }
}

#[async_trait]
impl UserQueryRepository for UserQueryRepositorySqlx {
    fn set_database(&mut self, database_name: &str) {
        self.current_database = database_name.to_string();
        // Nota: En una implementación completa, deberíamos cambiar el pool aquí
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        // Usar la macro query_as! de SQLx para mapeo de tipos con seguridad
        let record = sqlx::query!(
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
        .fetch_optional(self.base.pool())
        .await?;
        
        // Convertir a entidad de dominio si hay resultado
        Ok(record.map(|r| User {
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
        }))
    }
    
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let record = sqlx::query!(
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
            WHERE correo_electronico = $1
            "#,
            email
        )
        .fetch_optional(self.base.pool())
        .await?;
        
        Ok(record.map(|r| User {
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
        }))
    }
    
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let record = sqlx::query!(
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
            WHERE usuario = $1
            "#,
            username
        )
        .fetch_optional(self.base.pool())
        .await?;
        
        Ok(record.map(|r| User {
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
        }))
    }
    
    async fn find_all(&self) -> Result<Vec<User>> {
        let records = sqlx::query!(
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
            "#
        )
        .fetch_all(self.base.pool())
        .await?;
        
        Ok(records.into_iter().map(|r| User {
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
        }).collect())
    }
}