use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres, Row};
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
        // Reemplazar query! por query_as string para evitar la verificación estática
        let sql = r#"
            SELECT 
                idx_usuario as id,
                usuario as username, 
                nombre as first_name, 
                apellido as last_name,
                correo_electronico as email, 
                contraseña as password,
                status,
                creado_por as created_by,
                fecha_creacion as created_at,
                modificado_por as modified_by,
                fecha_modificacion as modified_at
            FROM usuarios 
            WHERE idx_usuario = $1
        "#;
        
        let result = sqlx::query(sql)
            .bind(id)
            .fetch_optional(self.base.pool())
            .await;
            
        match result {
            Ok(Some(row)) => {
                // Mapeo manual de filas a entidad
                let user = User {
                    id: row.try_get("id")?,
                    username: row.try_get("username")?,
                    first_name: row.try_get("first_name")?,
                    last_name: row.try_get("last_name")?,
                    email: row.try_get("email")?,
                    password: row.try_get("password")?,
                    status: row.try_get("status")?,
                    created_by: row.try_get("created_by")?,
                    created_at: row.try_get("created_at")?,
                    modified_by: row.try_get("modified_by")?,
                    modified_at: row.try_get("modified_at")?,
                };
                Ok(Some(user))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(anyhow!("Error al buscar usuario por ID: {}", e)),
        }
    }
    
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let sql = r#"
            SELECT 
                idx_usuario as id,
                usuario as username, 
                nombre as first_name, 
                apellido as last_name,
                correo_electronico as email, 
                contraseña as password,
                status,
                creado_por as created_by,
                fecha_creacion as created_at,
                modificado_por as modified_by,
                fecha_modificacion as modified_at
            FROM usuarios 
            WHERE correo_electronico = $1
        "#;
        
        let result = sqlx::query(sql)
            .bind(email)
            .fetch_optional(self.base.pool())
            .await;
            
        match result {
            Ok(Some(row)) => {
                // Mapeo manual de filas a entidad
                let user = User {
                    id: row.try_get("id")?,
                    username: row.try_get("username")?,
                    first_name: row.try_get("first_name")?,
                    last_name: row.try_get("last_name")?,
                    email: row.try_get("email")?,
                    password: row.try_get("password")?,
                    status: row.try_get("status")?,
                    created_by: row.try_get("created_by")?,
                    created_at: row.try_get("created_at")?,
                    modified_by: row.try_get("modified_by")?,
                    modified_at: row.try_get("modified_at")?,
                };
                Ok(Some(user))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(anyhow!("Error al buscar usuario por email: {}", e)),
        }
    }
    
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let sql = r#"
            SELECT 
                idx_usuario as id,
                usuario as username, 
                nombre as first_name, 
                apellido as last_name,
                correo_electronico as email, 
                contraseña as password,
                status,
                creado_por as created_by,
                fecha_creacion as created_at,
                modificado_por as modified_by,
                fecha_modificacion as modified_at
            FROM usuarios 
            WHERE usuario = $1
        "#;
        
        let result = sqlx::query(sql)
            .bind(username)
            .fetch_optional(self.base.pool())
            .await;
            
        match result {
            Ok(Some(row)) => {
                // Mapeo manual de filas a entidad
                let user = User {
                    id: row.try_get("id")?,
                    username: row.try_get("username")?,
                    first_name: row.try_get("first_name")?,
                    last_name: row.try_get("last_name")?,
                    email: row.try_get("email")?,
                    password: row.try_get("password")?,
                    status: row.try_get("status")?,
                    created_by: row.try_get("created_by")?,
                    created_at: row.try_get("created_at")?,
                    modified_by: row.try_get("modified_by")?,
                    modified_at: row.try_get("modified_at")?,
                };
                Ok(Some(user))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(anyhow!("Error al buscar usuario por username: {}", e)),
        }
    }
    
    async fn find_all(&self) -> Result<Vec<User>> {
        let sql = r#"
            SELECT 
                idx_usuario as id,
                usuario as username, 
                nombre as first_name, 
                apellido as last_name,
                correo_electronico as email, 
                contraseña as password,
                status,
                creado_por as created_by,
                fecha_creacion as created_at,
                modificado_por as modified_by,
                fecha_modificacion as modified_at
            FROM usuarios
        "#;
        
        let result = sqlx::query(sql)
            .fetch_all(self.base.pool())
            .await;
            
        match result {
            Ok(rows) => {
                let mut users = Vec::with_capacity(rows.len());
                for row in rows {
                    let user = User {
                        id: row.try_get("id")?,
                        username: row.try_get("username")?,
                        first_name: row.try_get("first_name")?,
                        last_name: row.try_get("last_name")?,
                        email: row.try_get("email")?,
                        password: row.try_get("password")?,
                        status: row.try_get("status")?,
                        created_by: row.try_get("created_by")?,
                        created_at: row.try_get("created_at")?,
                        modified_by: row.try_get("modified_by")?,
                        modified_at: row.try_get("modified_at")?,
                    };
                    users.push(user);
                }
                Ok(users)
            },
            Err(e) => Err(anyhow!("Error al buscar todos los usuarios: {}", e)),
        }
    }
}