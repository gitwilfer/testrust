use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use std::sync::Arc;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::Application::ports::repositories::UserCommandRepository;
use crate::Application::services::get_database_for_entity;
use crate::Domain::entities::user::User;
use crate::Infrastructure::Persistence::database::{self, DbConnection};
use crate::Infrastructure::Persistence::models::user_model::UserModel;
use crate::Infrastructure::Persistence::schema::users;
use crate::Infrastructure::Persistence::mapper::{user_to_model, model_to_user};

pub struct UserCommandRepositoryImpl {
    pool: Arc<Pool<ConnectionManager<diesel::PgConnection>>>,
}

impl UserCommandRepositoryImpl {
    pub fn new() -> Result<Self> {
        let pool = match database::get_default_connection() {
            Ok(conn) => {
                let conn_ref = conn;
                drop(conn_ref);
                Arc::new(database::get_pool_from_connection())
            },
            Err(e) => return Err(anyhow::anyhow!("No se pudo obtener el pool de conexiones principal: {}", e)),
        };
        
        Ok(Self { pool })
    }
    
    pub fn with_connection(conn: &mut PgConnection) -> Self {
        // Esta versión se usa dentro de transacciones
        // No necesita pool porque usa la conexión proporcionada
        Self { 
            pool: Arc::new(Pool::builder().build(ConnectionManager::new("")).unwrap()) 
        }
    }
    
    async fn get_connection(&self) -> Result<DbConnection> {
        let db_name = get_database_for_entity("user");
        Ok(database::get_connection(&db_name)?)
    }
}

#[async_trait]
impl UserCommandRepository for UserCommandRepositoryImpl {
    async fn create(&self, user: User) -> Result<User> {
        let mut conn = self.get_connection().await?;
        let user_model = user_to_model(&user);
        
        diesel::insert_into(users::table)
            .values(user_model)
            .execute(&mut conn)?;
        
        Ok(user)
    }

    async fn update(&self, user: User) -> Result<User> {
        let mut conn = self.get_connection().await?;
        let user_model = user_to_model(&user);
        
        diesel::update(users::table.filter(users::idx_usuario.eq(user.id)))
            .set(user_model)
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
}