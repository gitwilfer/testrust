// src/Infrastructure/repositories/user_repository_impl.rs
use async_trait::async_trait;
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::future::Future;
use std::pin::Pin;
use tokio::runtime::Handle;

use crate::application::ports::repositories::UserRepositoryPort;
use crate::application::services::{get_database_for_entity, get_default_database};
use crate::domain::entities::user::User;
use crate::infrastructure::persistence::database::{self, DbConnection};
use crate::infrastructure::persistence::models::user_model::UserModel;
use crate::infrastructure::persistence::schema::users;
use crate::infrastructure::persistence::mapper::{user_to_model, model_to_user};

// Definición del trait TransactionalUserRepository que faltaba
#[async_trait]
pub trait TransactionalUserRepository: Send + Sync {
    async fn transaction<F, Fut, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&dyn UserRepositoryPort) -> Fut + Send + 'static,
        Fut: Future<Output = Result<R>> + Send + 'static,
        R: Send + 'static;
        
    async fn create_in_transaction(&self, user: User) -> Result<User>;
    async fn update_in_transaction(&self, user: User) -> Result<User>;
}

pub struct UserRepositoryImpl {
    // Podemos guardar el pool de conexiones aquí para mayor eficiencia
    pool: Arc<Pool<ConnectionManager<diesel::PgConnection>>>,
}

impl UserRepositoryImpl {
    pub fn new() -> Result<Self> {
        // Obtenemos el pool de conexiones desde el gestor de bases de datos
        let pool = match database::get_default_connection() {
            Ok(conn) => {
                // Si podemos obtener una conexión, entonces tenemos acceso al pool
                let conn_ref = conn;
                drop(conn_ref); // Liberamos la conexión
                Arc::new(database::get_pool_from_connection())
            },
            Err(_) => return Err(anyhow::anyhow!("No se pudo obtener el pool de conexiones principal")),
        };
        
        Ok(Self { pool })
    }
    
    // Método auxiliar para obtener una conexión
    async fn get_connection(&self) -> Result<DbConnection> {
        let db_name = get_database_for_entity("user");
        database::get_connection(&db_name)
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
        // Solución: Ejecución de operaciones de transacción en el mismo hilo
        // usando un enfoque que garantiza que todo el trabajo se realice en el mismo contexto
        let mut conn = self.get_connection().await?;
        
        // Usamos block_in_place para ejecutar código sincrónico (la transacción)
        // que envolverá nuestro código asíncrono
        tokio::task::block_in_place(move || {
            // Creamos un struct que implementa UserRepositoryPort de forma sincrónica
            // El repository solo vivirá durante la transacción
            struct SyncUserRepositoryAdapter<'a> {
                conn: &'a mut PgConnection,
            }
            
            impl<'a> SyncUserRepositoryAdapter<'a> {
                fn new(conn: &'a mut PgConnection) -> Self {
                    Self { conn }
                }
                
                // Métodos sincronizados que replican la API asíncrona
                fn create(&self, user: User) -> Result<User> {
                    let user_model = user_to_model(&user);
                    diesel::insert_into(users::table)
                        .values(&user_model)
                        .execute(self.conn)?;
                    Ok(user)
                }
                
                fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
                    let result = users::table
                        .filter(users::idx_usuario.eq(id))
                        .first::<UserModel>(self.conn)
                        .optional()?;
                    Ok(result.map(|model| model_to_user(&model)))
                }
                
                fn find_by_email(&self, email: &str) -> Result<Option<User>> {
                    let result = users::table
                        .filter(users::correo_electronico.eq(email))
                        .first::<UserModel>(self.conn)
                        .optional()?;
                    Ok(result.map(|model| model_to_user(&model)))
                }
                
                fn find_by_username(&self, username: &str) -> Result<Option<User>> {
                    let result = users::table
                        .filter(users::usuario.eq(username))
                        .first::<UserModel>(self.conn)
                        .optional()?;
                    Ok(result.map(|model| model_to_user(&model)))
                }
                
                fn update(&self, user: User) -> Result<User> {
                    let user_model = user_to_model(&user);
                    diesel::update(users::table.filter(users::idx_usuario.eq(user.id)))
                        .set(&user_model)
                        .execute(self.conn)?;
                    Ok(user)
                }
                
                fn delete(&self, id: Uuid) -> Result<()> {
                    let affected = diesel::delete(users::table.filter(users::idx_usuario.eq(id)))
                        .execute(self.conn)?;
                    if affected == 0 {
                        return Err(anyhow::anyhow!("Usuario no encontrado"));
                    }
                    Ok(())
                }
                
                fn find_all(&self) -> Result<Vec<User>> {
                    let models = users::table
                        .load::<UserModel>(self.conn)?;
                    Ok(models.iter().map(|model| model_to_user(model)).collect())
                }
            }
            
            // Una estructura que adapta el SyncUserRepositoryAdapter a UserRepositoryPort asíncrono
            struct AsyncAdapterWrapper<'a> {
                sync_repo: SyncUserRepositoryAdapter<'a>,
            }
            
            #[async_trait]
            impl<'a> UserRepositoryPort for AsyncAdapterWrapper<'a> {
                async fn create(&self, user: User) -> Result<User> {
                    self.sync_repo.create(user)
                }
                
                async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
                    self.sync_repo.find_by_id(id)
                }
                
                async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
                    self.sync_repo.find_by_email(email)
                }
                
                async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
                    self.sync_repo.find_by_username(username)
                }
                
                async fn update(&self, user: User) -> Result<User> {
                    self.sync_repo.update(user)
                }
                
                async fn delete(&self, id: Uuid) -> Result<()> {
                    self.sync_repo.delete(id)
                }
                
                async fn find_all(&self) -> Result<Vec<User>> {
                    self.sync_repo.find_all()
                }
            }
            
            // Ejecutamos la transacción
            conn.transaction(|conn| {
                let sync_repo = SyncUserRepositoryAdapter::new(conn);
                let async_repo = AsyncAdapterWrapper { sync_repo };
                
                // Ejecutamos el futuro proporcionado por el usuario
                // dentro del runtime de Tokio actual
                Handle::current().block_on(async {
                    f(&async_repo).await
                })
            })
        })
    }
    
    async fn create_in_transaction(&self, user: User) -> Result<User> {
        let mut conn = self.get_connection().await?;
        let user_model = user_to_model(&user);
        let user_clone = user.clone();  // Clonamos para evitar problemas de lifetime
        
        conn.transaction(|conn| {
            diesel::insert_into(users::table)
                .values(&user_model)
                .execute(conn)
                .map_err(|e| anyhow::anyhow!("Error en transacción: {}", e))
                .map(|_| user_clone.clone())
        })
    }
    
    async fn update_in_transaction(&self, user: User) -> Result<User> {
        let mut conn = self.get_connection().await?;
        let user_model = user_to_model(&user);
        let user_clone = user.clone();  // Clonamos para evitar problemas de lifetime
        
        conn.transaction(|conn| {
            // En lugar de usar set(), actualizamos campo por campo
            diesel::update(users::table.filter(users::idx_usuario.eq(user.id)))
                .set((
                    users::nombre.eq(&user_model.first_name),
                    users::apellido.eq(&user_model.last_name),
                    users::correo_electronico.eq(&user_model.email),
                    users::password_hash.eq(&user_model.password),
                    users::modificado_por.eq(&user_model.modified_by),
                    users::fecha_modificacion.eq(&user_model.modified_at),
                    users::status.eq(&user_model.status),
                ))
                .execute(conn)
                .map_err(|e| anyhow::anyhow!("Error en transacción de actualización: {}", e))
                .map(|_| user_clone.clone())
        })
    }
}