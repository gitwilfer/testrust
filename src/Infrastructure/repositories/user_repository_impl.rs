// src/infrastructure/repositories/user_repository_impl.rs
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use std::sync::Arc;
use uuid::Uuid;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tokio::task;

use crate::application::ports::repositories::UserRepositoryPort;
use crate::application::services::{get_database_for_entity, get_default_database};
use crate::domain::entities::user::User;
use crate::infrastructure::persistence::database::{self, DbConnection};
use crate::infrastructure::persistence::models::user_model::UserModel;
use crate::infrastructure::persistence::schema::users;
use crate::infrastructure::persistence::mapper::{user_to_model, model_to_user};

// Definimos el trait TransactionalUserRepository
#[async_trait]
pub trait TransactionalUserRepository: Send + Sync {
    async fn transaction<F, Fut, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(Arc<dyn UserRepositoryPort>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<R>> + Send + 'static,
        R: Send + 'static;
    
    async fn create_in_transaction(&self, user: User) -> Result<User>;
    async fn update_in_transaction(&self, user: User) -> Result<User>;
}

pub struct UserRepositoryImpl {
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
            Err(e) => return Err(anyhow!("No se pudo obtener el pool de conexiones principal: {}", e)),
        };
        
        Ok(Self { pool })
    }
    
    // Método auxiliar para obtener una conexión
    async fn get_connection(&self) -> Result<DbConnection> {
        let db_name = get_database_for_entity("user");
        // Convertimos el error de r2d2 a anyhow
        database::get_connection(&db_name).map_err(|e| anyhow!("Error al obtener conexión: {}", e))
    }
}

// Implementación de funciones síncronas que utilizaremos dentro de las transacciones
impl UserRepositoryImpl {
    // Estas funciones son síncronas y se ejecutarán directamente en el hilo de la transacción
    fn sync_create(&self, conn: &mut PgConnection, user: User) -> Result<User> {
        let user_model = user_to_model(&user);
        
        diesel::insert_into(users::table)
            .values(&user_model)
            .execute(conn)?;
        
        Ok(user)
    }
    
    fn sync_find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<User>> {
        let result = users::table
            .filter(users::idx_usuario.eq(id))
            .first::<UserModel>(conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    fn sync_find_by_email(&self, conn: &mut PgConnection, email: &str) -> Result<Option<User>> {
        let result = users::table
            .filter(users::correo_electronico.eq(email))
            .first::<UserModel>(conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    fn sync_find_by_username(&self, conn: &mut PgConnection, username: &str) -> Result<Option<User>> {
        let result = users::table
            .filter(users::usuario.eq(username))
            .first::<UserModel>(conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    fn sync_update(&self, conn: &mut PgConnection, user: User) -> Result<User> {
        let user_model = user_to_model(&user);
        
        diesel::update(users::table.filter(users::idx_usuario.eq(user.id)))
            .set(&user_model)
            .execute(conn)?;
        
        Ok(user)
    }
    
    fn sync_delete(&self, conn: &mut PgConnection, id: Uuid) -> Result<()> {
        let affected = diesel::delete(users::table.filter(users::idx_usuario.eq(id)))
            .execute(conn)?;
        
        if affected == 0 {
            return Err(anyhow!("Usuario no encontrado"));
        }
        
        Ok(())
    }
    
    fn sync_find_all(&self, conn: &mut PgConnection) -> Result<Vec<User>> {
        let models = users::table
            .load::<UserModel>(conn)?;
        
        Ok(models.iter().map(|model| model_to_user(model)).collect())
    }
}

#[async_trait]
impl UserRepositoryPort for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<User> {
        let mut conn = self.get_connection().await?;
        
        // Ejecutamos la función síncrona en un contexto de bloqueo
        task::block_in_place(move || {
            self.sync_create(&mut conn, user)
        })
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let mut conn = self.get_connection().await?;
        
        task::block_in_place(move || {
            self.sync_find_by_id(&mut conn, id)
        })
    }
    
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let email = email.to_string(); // Clonamos el string para llevarlo al otro hilo
        let mut conn = self.get_connection().await?;
        
        task::block_in_place(move || {
            self.sync_find_by_email(&mut conn, &email)
        })
    }
    
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let username = username.to_string(); // Clonamos el string para llevarlo al otro hilo
        let mut conn = self.get_connection().await?;
        
        task::block_in_place(move || {
            self.sync_find_by_username(&mut conn, &username)
        })
    }
    
    async fn update(&self, user: User) -> Result<User> {
        let mut conn = self.get_connection().await?;
        
        task::block_in_place(move || {
            self.sync_update(&mut conn, user)
        })
    }
    
    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        task::block_in_place(move || {
            self.sync_delete(&mut conn, id)
        })
    }
    
    async fn find_all(&self) -> Result<Vec<User>> {
        let mut conn = self.get_connection().await?;
        
        task::block_in_place(move || {
            self.sync_find_all(&mut conn)
        })
    }
}

// Implementación para soporte de transacciones
#[async_trait]
impl TransactionalUserRepository for UserRepositoryImpl {
    async fn transaction<F, Fut, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(Arc<dyn UserRepositoryPort>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<R>> + Send + 'static,
        R: Send + 'static
    {
        let mut conn = self.get_connection().await?;
        let self_clone = Arc::new(Self { 
            pool: self.pool.clone() 
        });
        
        // Crear un repositorio de una sola transacción
        struct TransactionRepo {
            repo: Arc<UserRepositoryImpl>,
            conn: DbConnection,
        }
        
        #[async_trait]
        impl UserRepositoryPort for TransactionRepo {
            async fn create(&self, user: User) -> Result<User> {
                let mut conn = &mut *self.conn;
                task::block_in_place(move || {
                    self.repo.sync_create(&mut conn, user)
                })
            }
            
            async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
                let mut conn = &mut *self.conn;
                task::block_in_place(move || {
                    self.repo.sync_find_by_id(&mut conn, id)
                })
            }
            
            async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
                let email = email.to_string();
                let mut conn = &mut *self.conn;
                task::block_in_place(move || {
                    self.repo.sync_find_by_email(&mut conn, &email)
                })
            }
            
            async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
                let username = username.to_string();
                let mut conn = &mut *self.conn;
                task::block_in_place(move || {
                    self.repo.sync_find_by_username(&mut conn, &username)
                })
            }
            
            async fn update(&self, user: User) -> Result<User> {
                let mut conn = &mut *self.conn;
                task::block_in_place(move || {
                    self.repo.sync_update(&mut conn, user)
                })
            }
            
            async fn delete(&self, id: Uuid) -> Result<()> {
                let mut conn = &mut *self.conn;
                task::block_in_place(move || {
                    self.repo.sync_delete(&mut conn, id)
                })
            }
            
            async fn find_all(&self) -> Result<Vec<User>> {
                let mut conn = &mut *self.conn;
                task::block_in_place(move || {
                    self.repo.sync_find_all(&mut conn)
                })
            }
        }
        
        // Ejecutamos la transacción en un task bloqueante para no bloquear el runtime async
        task::block_in_place(move || {
            let result = conn.transaction::<R, anyhow::Error, _>(|conn_ref| {
                // Creamos un repositorio de transacción
                let tx_repo = Arc::new(TransactionRepo {
                    repo: self_clone,
                    conn: conn_ref,
                });
                
                // Ejecutamos la función del usuario en un nuevo runtime
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                    
                rt.block_on(f(tx_repo))
            });
            
            result
        })
    }
    
    async fn create_in_transaction(&self, user: User) -> Result<User> {
        let mut conn = self.get_connection().await?;
        let user_clone = user.clone();  // Clonamos para evitar problemas de ownership
        
        task::block_in_place(move || {
            conn.transaction(|conn| {
                self.sync_create(conn, user_clone)
            })
        })
    }
    
    async fn update_in_transaction(&self, user: User) -> Result<User> {
        let mut conn = self.get_connection().await?;
        let user_clone = user.clone();
        
        task::block_in_place(move || {
            conn.transaction(|conn| {
                self.sync_update(conn, user_clone)
            })
        })
    }
}