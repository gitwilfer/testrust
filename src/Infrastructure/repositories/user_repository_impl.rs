// src/infrastructure/repositories/user_repository_impl.rs
use async_trait::async_trait;
use anyhow::Result;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use uuid::Uuid;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::connection::TransactionManager;

use crate::application::ports::repositories::{UserRepositoryPort, TransactionalUserRepository};
use crate::application::services::{get_database_for_entity, get_default_database};
use crate::domain::entities::user::User;
use crate::infrastructure::persistence::database::{self, DbConnection};
use crate::infrastructure::persistence::models::user_model::UserModel;
use crate::infrastructure::persistence::schema::users;
use crate::infrastructure::persistence::mapper::{user_to_model, model_to_user};

pub struct UserRepositoryImpl {
    // Podemos guardar el pool de conexiones aquí para mayor eficiencia
    pool: Arc<Pool<ConnectionManager<diesel::PgConnection>>>,
}

impl UserRepositoryImpl {
    pub fn new() -> Result<Self> {
        // Obtenemos el pool de conexiones desde el gestor de bases de datos
        let pool = match database::get_pool("main") {
            Some(pool) => Arc::new(pool.clone()),
            None => return Err(anyhow::anyhow!("No se pudo obtener el pool de conexiones principal")),
        };
        
        Ok(Self { pool })
    }
    
    // Método auxiliar para obtener una conexión
    async fn get_connection(&self) -> Result<DbConnection> {
        Ok(self.pool.get()?)
    }

    // Método para ejecutar una operación dentro de una transacción
    async fn with_transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut DbConnection) -> Result<R>,
    {
        let mut conn = self.get_connection().await?;
        
        conn.transaction(|conn| {
            f(conn)
        })
    }
}

#[async_trait]
impl UserRepositoryPort for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<User> {
        let mut conn = self.get_connection().await?;
        let user_model = user_to_model(&user);
        
        diesel::insert_into(users::table)
            .values(&user_model)
            .execute(&mut conn)?;
        
        Ok(user)
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let mut conn = self.get_connection().await?;
        
        let result = users::table
            .filter(users::idx_usuario.eq(id))
            .first::<UserModel>(&mut conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let mut conn = self.get_connection().await?;
        
        let result = users::table
            .filter(users::correo_electronico.eq(email))
            .first::<UserModel>(&mut conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let mut conn = self.get_connection().await?;
        
        let result = users::table
            .filter(users::usuario.eq(username))
            .first::<UserModel>(&mut conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    async fn update(&self, user: User) -> Result<User> {
        let mut conn = self.get_connection().await?;
        let user_model = user_to_model(&user);
        
        diesel::update(users::table.filter(users::idx_usuario.eq(user.id)))
            .set(&user_model)
            .execute(&mut conn)?;
        
        Ok(user)
    }
    
    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        let affected = diesel::delete(users::table.filter(users::idx_usuario.eq(id)))
            .execute(&mut conn)?;
        
        if affected == 0 {
            return Err(anyhow::anyhow!("Usuario no encontrado"));
        }
        
        Ok(())
    }
    
    async fn find_all(&self) -> Result<Vec<User>> {
        let mut conn = self.get_connection().await?;
        
        let models = users::table
            .load::<UserModel>(&mut conn)?;
        
        Ok(models.iter().map(|model| model_to_user(model)).collect())
    }
}

// Implementación para soporte de transacciones
#[async_trait]
impl TransactionalUserRepository for UserRepositoryImpl {
    async fn transaction<F, Fut, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&dyn UserRepositoryPort) -> Fut + Send + 'static,
        Fut: Future<Output = Result<R>> + Send + 'static,
        R: Send + 'static
    {
        // Crear un repositorio temporal para la transacción
        struct TransactionUserRepository<'a> {
            conn: &'a mut DbConnection,
        }
        
        #[async_trait]
        impl<'a> UserRepositoryPort for TransactionUserRepository<'a> {
            async fn create(&self, user: User) -> Result<User> {
                let user_model = user_to_model(&user);
                
                diesel::insert_into(users::table)
                    .values(&user_model)
                    .execute(self.conn)?;
                
                Ok(user)
            }
            
            async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
                let result = users::table
                    .filter(users::idx_usuario.eq(id))
                    .first::<UserModel>(self.conn)
                    .optional()?;
                
                Ok(result.map(|model| model_to_user(&model)))
            }
            
            async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
                let result = users::table
                    .filter(users::correo_electronico.eq(email))
                    .first::<UserModel>(self.conn)
                    .optional()?;
                
                Ok(result.map(|model| model_to_user(&model)))
            }
            
            async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
                let result = users::table
                    .filter(users::usuario.eq(username))
                    .first::<UserModel>(self.conn)
                    .optional()?;
                
                Ok(result.map(|model| model_to_user(&model)))
            }
            
            async fn update(&self, user: User) -> Result<User> {
                let user_model = user_to_model(&user);
                
                diesel::update(users::table.filter(users::idx_usuario.eq(user.id)))
                    .set(&user_model)
                    .execute(self.conn)?;
                
                Ok(user)
            }
            
            async fn delete(&self, id: Uuid) -> Result<()> {
                let affected = diesel::delete(users::table.filter(users::idx_usuario.eq(id)))
                    .execute(self.conn)?;
                
                if affected == 0 {
                    return Err(anyhow::anyhow!("Usuario no encontrado"));
                }
                
                Ok(())
            }
            
            async fn find_all(&self) -> Result<Vec<User>> {
                let models = users::table
                    .load::<UserModel>(self.conn)?;
                
                Ok(models.iter().map(|model| model_to_user(model)).collect())
            }
        }
        
        // Obtener conexión y comenzar transacción
        let mut conn = self.get_connection().await?;
        let result = conn.transaction(|c| {
            let repo = TransactionUserRepository { conn: c };
            
            // Ejecutar la función dentro de la transacción
            // (en un runtime de Tokio para manejar las operaciones async)
            Box::pin(async move {
                f(&repo).await
            })
        });
        
        result
    }
}