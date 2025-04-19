use async_trait::async_trait;
use anyhow::{Result, anyhow, Context}; // Añadir Context para errores
use uuid::Uuid;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl}; // <-- AÑADIDO: Imports Async
use chrono::Utc;

use crate::Application::ports::driven::repositories::UserCommandRepository;

use crate::Domain::entities::user::User;
use crate::Infrastructure::Persistence::models::user_model::UpdateUserChangeset;
use crate::Infrastructure::Persistence::schema::users;
use crate::Infrastructure::Persistence::mapper::user_to_model;

// --- CONVERTIDO A ZST ---
#[derive(Clone, Copy)] // Añadir derives para ZST
pub struct UserCommandRepositoryImpl;

impl UserCommandRepositoryImpl {
    // Constructor simple para ZST
    pub fn new() -> Self {
        Self
    }
    // --- ELIMINADO ---
    // pub fn with_connection(conn: &mut PgConnection) -> Self { ... }
    // async fn get_connection(&self) -> Result<DbConnection> { ... }
    // -----------------
}

#[async_trait]
impl UserCommandRepository for UserCommandRepositoryImpl {
    // --- AJUSTADO: Acepta 'conn: &mut AsyncPgConnection' ---
    async fn create(&self, conn: &mut AsyncPgConnection, user: User) -> Result<User> {
        let user_model = user_to_model(&user);

        diesel::insert_into(users::table)
            .values(user_model)
            .execute(conn) // Usar la conexión async pasada
            .await // Usar .await para la ejecución async
            .context("Failed to insert user using Diesel Async")?; // Añadir contexto de error

        // Devolver la entidad original (o podrías querer devolver la insertada si la BD genera campos)
        Ok(user)
    }

    // --- AJUSTADO: Acepta 'conn: &mut AsyncPgConnection' ---
    async fn update(&self, conn: &mut AsyncPgConnection, user: User) -> Result<User> {
        let now_utc = Utc::now();
        let changeset = UpdateUserChangeset {
            username: Some(user.username.clone()),
            first_name: Some(user.first_name.clone()),
            last_name: Some(user.last_name.clone()),
            email: Some(user.email.clone()),
            password: Some(user.password.clone()),
            updated_by: user.updated_by,
            updated_at: Some(now_utc.naive_utc()),
            status: Some(user.status),
        };

        let affected_rows = diesel::update(users::table.find(user.id))
            .set(&changeset)
            .execute(conn) // Usar la conexión async pasada
            .await // Usar .await para la ejecución async
            .context(format!("Failed to update user {} using Diesel Async", user.id))?; // Añadir contexto

        if affected_rows == 0 {
            return Err(anyhow!("Usuario con ID {} no encontrado para actualizar", user.id));
        }
        Ok(user)
    }

    // --- AJUSTADO: Acepta 'conn: &mut AsyncPgConnection' ---
    async fn delete(&self, conn: &mut AsyncPgConnection, id: Uuid) -> Result<()> {
        // --- ELIMINADO ---
        // let mut conn = self.get_connection().await?;
        // -----------------

        let affected = diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(conn) // Usar la conexión async pasada
            .await // Usar .await para la ejecución async
            .context(format!("Failed to delete user {} using Diesel Async", id))?; // Añadir contexto

        if affected == 0 {
            // Considera si no encontrarlo es realmente un error en tu caso de uso
            return Err(anyhow!("Usuario {} no encontrado para eliminar", id));
        }

        Ok(())
    }
}
