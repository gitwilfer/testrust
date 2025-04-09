use async_trait::async_trait;
use anyhow::{Result, anyhow};
use std::sync::Arc;
use uuid::Uuid;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tokio::task;
use std::future::Future;
use std::pin::Pin;

use crate::application::ports::repositories::UserRepositoryPort;
use crate::application::services::{get_database_for_entity, get_default_database};
use crate::domain::entities::user::User;
use crate::infrastructure::persistence::database::{self, DbConnection};
use crate::infrastructure::persistence::models::user_model::UserModel;
use crate::infrastructure::persistence::schema::users;
use crate::infrastructure::persistence::mapper::{user_to_model, model_to_user};

// Definimos un trait para transacciones que no tiene la restricción Sync
// Esto es clave para nuestra solución
#[async_trait]
pub trait TransactionalOperations: Send {
    async fn execute_in_transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut PgConnection) -> Result<R> + Send + 'static,
        R: Send + 'static;
}

// Trait que sí extiende UserRepositoryPort pero con funcionalidad reducida para transacciones
#[async_trait]
pub trait TransactionalUserRepository: UserRepositoryPort {
    async fn create_in_transaction(&self, user: User) -> Result<User>;
    async fn update_in_transaction(&self, user: User) -> Result<User>;
}

pub struct UserRepositoryImpl {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl UserRepositoryImpl {
    pub fn new() -> Result<Self> {
        let pool = match database::get_default_connection() {
            Ok(conn) => {
                let conn_ref = conn;
                drop(conn_ref);
                Arc::new(database::get_pool_from_connection())
            },
            Err(e) => return Err(anyhow!("No se pudo obtener el pool de conexiones principal: {}", e)),
        };
        
        Ok(Self { pool })
    }
    
    async fn get_connection(&self) -> Result<DbConnection> {
        let db_name = get_database_for_entity("user");
        database::get_connection(&db_name)
            .map_err(|e| anyhow!("Error al obtener conexión: {}", e))
    }
    
    // Funciones síncronas para operaciones con la base de datos
    fn sync_create(user: User, conn: &mut PgConnection) -> Result<User> {
        let user_model = user_to_model(&user);
        
        diesel::insert_into(users::table)
            .values(&user_model)
            .execute(conn)?;
        
        Ok(user)
    }
    
    fn sync_find_by_id(id: Uuid, conn: &mut PgConnection) -> Result<Option<User>> {
        let result = users::table
            .filter(users::idx_usuario.eq(id))
            .first::<UserModel>(conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    fn sync_find_by_email(email: &str, conn: &mut PgConnection) -> Result<Option<User>> {
        let result = users::table
            .filter(users::correo_electronico.eq(email))
            .first::<UserModel>(conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    fn sync_find_by_username(username: &str, conn: &mut PgConnection) -> Result<Option<User>> {
        let result = users::table
            .filter(users::usuario.eq(username))
            .first::<UserModel>(conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    fn sync_update(user: User, conn: &mut PgConnection) -> Result<User> {
        let user_model = user_to_model(&user);
        
        diesel::update(users::table.filter(users::idx_usuario.eq(user.id)))
            .set(&user_model)
            .execute(conn)?;
        
        Ok(user)
    }
    
    fn sync_delete(id: Uuid, conn: &mut PgConnection) -> Result<()> {
        let affected = diesel::delete(users::table.filter(users::idx_usuario.eq(id)))
            .execute(conn)?;
        
        if affected == 0 {
            return Err(anyhow!("Usuario no encontrado"));
        }
        
        Ok(())
    }
    
    fn sync_find_all(conn: &mut PgConnection) -> Result<Vec<User>> {
        let models = users::table
            .load::<UserModel>(conn)?;
        
        Ok(models.iter().map(|model| model_to_user(model)).collect())
    }
}

// Implementamos TransactionalOperations para ejecutar funciones en una transacción
#[async_trait]
impl TransactionalOperations for UserRepositoryImpl {
    async fn execute_in_transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut PgConnection) -> Result<R> + Send + 'static,
        R: Send + 'static,
    {
        let mut conn = self.get_connection().await?;
        
        // Ejecutamos la transacción en un contexto bloqueante
        task::block_in_place(move || {
            conn.transaction(f)
        })
    }
}

#[async_trait]
impl UserRepositoryPort for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<User> {
        let user_clone = user.clone();
        self.execute_in_transaction(move |conn| {
            Self::sync_create(user_clone, conn)
        }).await
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let id_clone = id.clone();
        self.execute_in_transaction(move |conn| {
            Self::sync_find_by_id(id_clone, conn)
        }).await
    }
    
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let email_clone = email.to_string();
        self.execute_in_transaction(move |conn| {
            Self::sync_find_by_email(&email_clone, conn)
        }).await
    }
    
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let username_clone = username.to_string();
        self.execute_in_transaction(move |conn| {
            Self::sync_find_by_username(&username_clone, conn)
        }).await
    }
    
    async fn update(&self, user: User) -> Result<User> {
        let user_clone = user.clone();
        self.execute_in_transaction(move |conn| {
            Self::sync_update(user_clone, conn)
        }).await
    }
    
    async fn delete(&self, id: Uuid) -> Result<()> {
        let id_clone = id.clone();
        self.execute_in_transaction(move |conn| {
            Self::sync_delete(id_clone, conn)
        }).await
    }
    
    async fn find_all(&self) -> Result<Vec<User>> {
        self.execute_in_transaction(move |conn| {
            Self::sync_find_all(conn)
        }).await
    }
}

// Implementación para soporte de transacciones simplificado
#[async_trait]
impl TransactionalUserRepository for UserRepositoryImpl {
    async fn create_in_transaction(&self, user: User) -> Result<User> {
        self.create(user).await
    }
    
    async fn update_in_transaction(&self, user: User) -> Result<User> {
        self.update(user).await
    }
}