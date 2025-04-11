use async_trait::async_trait;
use anyhow::{Result, anyhow, Context};
use std::sync::Arc;
use uuid::Uuid;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use actix_web::web;
use log::{error, debug};

use crate::Application::ports::repositories::UserRepositoryPort;
use crate::Application::services::{get_database_for_entity, get_default_database};
use crate::Domain::entities::user::User;
use crate::Infrastructure::Persistence::database::{self, DbConnection};
use crate::Infrastructure::Persistence::models::user_model::UserModel;
use crate::Infrastructure::Persistence::schema::users;
use crate::Infrastructure::Persistence::mapper::{user_to_model, model_to_user};

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
        debug!("Creando usuario en base de datos: {}", user.id);
        let user_model = user_to_model(&user);
        
        diesel::insert_into(users::table)
            .values(&user_model)
            .execute(conn)?;
        
        Ok(user)
    }
    
    fn sync_find_by_id(id: Uuid, conn: &mut PgConnection) -> Result<Option<User>> {
        debug!("Buscando usuario por ID: {}", id);
        let result = users::table
            .filter(users::idx_usuario.eq(id))
            .first::<UserModel>(conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    fn sync_find_by_email(email: &str, conn: &mut PgConnection) -> Result<Option<User>> {
        debug!("Buscando usuario por email: {}", email);
        let result = users::table
            .filter(users::correo_electronico.eq(email))
            .first::<UserModel>(conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    fn sync_find_by_username(username: &str, conn: &mut PgConnection) -> Result<Option<User>> {
        debug!("Buscando usuario por username: {}", username);
        let result = users::table
            .filter(users::usuario.eq(username))
            .first::<UserModel>(conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    fn sync_update(user: User, conn: &mut PgConnection) -> Result<User> {
        debug!("Actualizando usuario en base de datos: {}", user.id);
        let user_model = user_to_model(&user);
        
        diesel::update(users::table.filter(users::idx_usuario.eq(user.id)))
            .set(&user_model)
            .execute(conn)?;
        
        Ok(user)
    }
    
    fn sync_delete(id: Uuid, conn: &mut PgConnection) -> Result<()> {
        debug!("Eliminando usuario: {}", id);
        let affected = diesel::delete(users::table.filter(users::idx_usuario.eq(id)))
            .execute(conn)?;
        
        if affected == 0 {
            return Err(anyhow!("Usuario no encontrado"));
        }
        
        Ok(())
    }
    
    fn sync_find_all(conn: &mut PgConnection) -> Result<Vec<User>> {
        debug!("Obteniendo todos los usuarios");
        let models = users::table
            .load::<UserModel>(conn)?;
        
        Ok(models.iter().map(|model| model_to_user(model)).collect())
    }
    
    // Función genérica para ejecutar operaciones en transacción
    async fn execute_db_transaction<F, T>(&self, operation: F) -> Result<T>
    where
        F: FnOnce(&mut PgConnection) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        // Obtener pool de conexiones
        let pool = self.pool.clone();
        
        // Ejecutar operación en un hilo separado para no bloquear
        let result = web::block(move || {
            let mut conn = pool.get()?;
            
            // Ejecutar la operación dentro de una transacción
            conn.transaction(|conn| {
                match operation(conn) {
                    Ok(value) => Ok(value),
                    Err(e) => {
                        error!("Error durante la transacción: {:?}", e);
                        Err(e)
                    }
                }
            })
        })
        .await
        .map_err(|e| {
            error!("Error al ejecutar transacción: {:?}", e);
            anyhow!("Error de base de datos: {}", e)
        })?;
        
        Ok(result)
    }
}

#[async_trait]
impl UserRepositoryPort for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<User> {
        debug!("Iniciando creación de usuario: {}", user.id);
        let user_clone = user.clone();
        
        self.execute_db_transaction(move |conn| {
            Self::sync_create(user_clone, conn)
        }).await
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        debug!("Iniciando búsqueda de usuario por ID: {}", id);
        let id_clone = id.clone();
        
        self.execute_db_transaction(move |conn| {
            Self::sync_find_by_id(id_clone, conn)
        }).await
    }
    
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        debug!("Iniciando búsqueda de usuario por email: {}", email);
        let email_clone = email.to_string();
        
        self.execute_db_transaction(move |conn| {
            Self::sync_find_by_email(&email_clone, conn)
        }).await
    }
    
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        debug!("Iniciando búsqueda de usuario por username: {}", username);
        let username_clone = username.to_string();
        
        self.execute_db_transaction(move |conn| {
            Self::sync_find_by_username(&username_clone, conn)
        }).await
    }
    
    async fn update(&self, user: User) -> Result<User> {
        debug!("Iniciando actualización de usuario: {}", user.id);
        let user_clone = user.clone();
        
        self.execute_db_transaction(move |conn| {
            Self::sync_update(user_clone, conn)
        }).await
    }
    
    async fn delete(&self, id: Uuid) -> Result<()> {
        debug!("Iniciando eliminación de usuario: {}", id);
        let id_clone = id.clone();
        
        self.execute_db_transaction(move |conn| {
            Self::sync_delete(id_clone, conn)
        }).await
    }
    
    async fn find_all(&self) -> Result<Vec<User>> {
        debug!("Iniciando obtención de todos los usuarios");
        
        self.execute_db_transaction(move |conn| {
            Self::sync_find_all(conn)
        }).await
    }
}