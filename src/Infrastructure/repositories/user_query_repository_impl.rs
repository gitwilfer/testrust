use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use std::sync::Arc;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::Application::ports::repositories::UserQueryRepository;
use crate::Application::services::get_database_for_entity;
use crate::Domain::entities::user::User;
use crate::Infrastructure::Persistence::database::{self, DbConnection};
use crate::Infrastructure::Persistence::models::user_model::UserModel;
use crate::Infrastructure::Persistence::schema::users;
use crate::Infrastructure::Persistence::mapper::model_to_user;

pub struct UserQueryRepositoryImpl {
    pool: Arc<Pool<ConnectionManager<diesel::PgConnection>>>,
    database_name: String,
}

impl UserQueryRepositoryImpl {
    pub fn new() -> Result<Self> {
        let pool = match database::get_default_connection() {
            Ok(conn) => {
                let conn_ref = conn;
                drop(conn_ref);
                Arc::new(database::get_pool_from_connection())
            },
            Err(_) => return Err(anyhow::anyhow!("No se pudo obtener el pool de conexiones principal")),
        };
        
        Ok(Self { 
            pool, 
            database_name: "main".to_string() 
        })
    }
    
    async fn get_connection(&self) -> Result<DbConnection> {
        let db_name = get_database_for_entity("user");
        Ok(database::get_connection(&db_name)?)
    }
}

#[async_trait]
impl UserQueryRepository for UserQueryRepositoryImpl {
    fn set_database(&mut self, database_name: &str) {
        self.database_name = database_name.to_string();
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
        let email = email.to_string();
        let mut conn = self.get_connection().await?;
        
        let result = users::table
            .filter(users::correo_electronico.eq(email))
            .first::<UserModel>(&mut conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let username = username.to_string();
        let mut conn = self.get_connection().await?;
        
        let result = users::table
            .filter(users::usuario.eq(username))
            .first::<UserModel>(&mut conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    async fn find_all(&self) -> Result<Vec<User>> {
        let mut conn = self.get_connection().await?;
        
        let models = users::table
            .load::<UserModel>(&mut conn)?;
        
        Ok(models.iter().map(|model| model_to_user(model)).collect())
    }
}
