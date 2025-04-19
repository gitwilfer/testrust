use anyhow::{Result, anyhow};
use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc}; // <-- *** AÑADIDO IMPORT ***

use crate::Application::ports::driven::repositories::UserQueryRepository;
use crate::Domain::entities::user::User; // La struct User ya usa DateTime<Utc>
use crate::Infrastructure::repositories::sqlx_repository_base::SqlxRepositoryBase;

/// Requiere que se le inyecte el pool de conexiones SQLx.
pub struct UserQueryRepositorySqlx {
    base: SqlxRepositoryBase,
}

impl UserQueryRepositorySqlx {
    /// Constructor Preferido: Recibe el pool (Inyección de Dependencias).
    pub fn with_pool(pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            base: SqlxRepositoryBase::with_pool(pool, "user"),
        }
    }
}

#[async_trait]
impl UserQueryRepository for UserQueryRepositorySqlx {
    fn set_database(&mut self, database_name: &str) {
        log::warn!("Llamada a UserQueryRepositorySqlx::set_database('{}') ignorada...", database_name);
    }

    /// Busca un usuario por su ID.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let sql = r#"
            SELECT
                id, username, first_name, last_name, email,
                password_hash as password, status, created_by, created_at,
                updated_by, updated_at
            FROM users
            WHERE id = $1 AND status = 1
        "#;

        let result = sqlx::query(sql)
            .bind(id)
            .fetch_optional(self.base.pool())
            .await;

        match result {
            Ok(Some(row)) => {
                // El mapeo manual ahora debería funcionar si las features están activas
                let user = User {
                    id: row.try_get("id")?,
                    username: row.try_get("username")?,
                    first_name: row.try_get("first_name")?,
                    last_name: row.try_get("last_name")?,
                    email: row.try_get("email")?,
                    password: row.try_get("password")?,
                    status: row.try_get("status")?,
                    created_by: row.try_get("created_by")?,
                    created_at: row.try_get("created_at")?, // Requiere feature chrono
                    updated_by: row.try_get("updated_by")?,
                    updated_at: row.try_get("updated_at")?, // Requiere feature chrono
                };
                Ok(Some(user))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(anyhow!("Error en find_by_id: {}", e)),
        }
    }

    /// Busca un usuario por su email.
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let sql = r#" SELECT ... FROM users WHERE email = $1 AND status = 1 "#; // Query como antes

        let result = sqlx::query(sql)
            .bind(email)
            .fetch_optional(self.base.pool())
            .await;

        match result {
            Ok(Some(row)) => {
                let user = User {
                    id: row.try_get("id")?, username: row.try_get("username")?, first_name: row.try_get("first_name")?,
                    last_name: row.try_get("last_name")?, email: row.try_get("email")?, password: row.try_get("password")?,
                    status: row.try_get("status")?, created_by: row.try_get("created_by")?, created_at: row.try_get("created_at")?,
                    updated_by: row.try_get("updated_by")?, updated_at: row.try_get("updated_at")?,
                 };
                Ok(Some(user))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(anyhow!("Error en find_by_email: {}", e)),
        }
    }

    /// Busca un usuario por su nombre de usuario.
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let sql = r#" SELECT ... FROM users WHERE username = $1 AND status = 1 "#; // Query como antes

        let result = sqlx::query(sql)
            .bind(username)
            .fetch_optional(self.base.pool())
            .await;

        match result {
            Ok(Some(row)) => {
                let user = User {
                    id: row.try_get("id")?, username: row.try_get("username")?, first_name: row.try_get("first_name")?,
                    last_name: row.try_get("last_name")?, email: row.try_get("email")?, password: row.try_get("password")?,
                    status: row.try_get("status")?, created_by: row.try_get("created_by")?, created_at: row.try_get("created_at")?,
                    updated_by: row.try_get("updated_by")?, updated_at: row.try_get("updated_at")?,
                 };
                Ok(Some(user))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(anyhow!("Error en find_by_username: {}", e)),
        }
    }

    /// Obtiene todos los usuarios.
    async fn find_all(&self) -> Result<Vec<User>> {
        let sql = r#" SELECT ... FROM users WHERE status = 1 ORDER BY username "#; // Query como antes

        let result = sqlx::query(sql)
            .fetch_all(self.base.pool())
            .await;

        match result {
            Ok(rows) => {
                let mut users = Vec::with_capacity(rows.len());
                for row in rows {
                    let user = User {
                        id: row.try_get("id")?, username: row.try_get("username")?, first_name: row.try_get("first_name")?,
                        last_name: row.try_get("last_name")?, email: row.try_get("email")?, password: row.try_get("password")?,
                        status: row.try_get("status")?, created_by: row.try_get("created_by")?, created_at: row.try_get("created_at")?,
                        updated_by: row.try_get("updated_by")?, updated_at: row.try_get("updated_at")?,
                     };
                    users.push(user);
                }
                Ok(users)
            },
            Err(e) => Err(anyhow!("Error en find_all: {}", e)),
        }
    }
}
