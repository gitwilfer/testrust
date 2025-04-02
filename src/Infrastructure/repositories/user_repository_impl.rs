use crate::application::ports::repositories::UserRepositoryPort;
use crate::domain::entities::user::User;
use crate::infrastructure::persistence::database;
use crate::infrastructure::persistence::models::user_model::UserModel;
use crate::infrastructure::persistence::schema::users;
use crate::infrastructure::persistence::mapper::{user_to_model, model_to_user};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use anyhow::Result;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

pub struct UserRepositoryImpl;

#[async_trait]
impl UserRepositoryPort for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<User> {
        let mut conn = database::get_main_connection()?;
        let user_model = user_to_model(&user);
        
        diesel::insert_into(users::table)
            .values(&user_model)
            .execute(&mut conn)?;
        
        Ok(user)
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let mut conn = database::get_main_connection()?;
        
        let result = users::table
            .filter(users::idx_usuario.eq(id))
            .first::<UserModel>(&mut conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let mut conn = database::get_main_connection()?;
        
        let result = users::table
            .filter(users::correo_electronico.eq(email))
            .first::<UserModel>(&mut conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let mut conn = database::get_main_connection()?;
        
        let result = users::table
            .filter(users::usuario.eq(username))
            .first::<UserModel>(&mut conn)
            .optional()?;
        
        Ok(result.map(|model| model_to_user(&model)))
    }
    
    async fn update(&self, user: User) -> Result<User> {
        let mut conn = database::get_main_connection()?;
        let user_model = user_to_model(&user);
        
        diesel::update(users::table.filter(users::idx_usuario.eq(user.id)))
            .set(&user_model)
            .execute(&mut conn)?;
        
        Ok(user)
    }
    
    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut conn = database::get_main_connection()?;
        
        let affected = diesel::delete(users::table.filter(users::idx_usuario.eq(id)))
            .execute(&mut conn)?;
        
        if affected == 0 {
            return Err(anyhow::anyhow!("Usuario no encontrado"));
        }
        
        Ok(())
    }
    
    async fn find_all(&self) -> Result<Vec<User>> {
        let mut conn = database::get_main_connection()?;
        
        let models = users::table
            .load::<UserModel>(&mut conn)?;
        
        Ok(models.iter().map(|model| model_to_user(model)).collect())
    }
    
    async fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(Box<dyn UserRepositoryPort + '_>) -> Pin<Box<dyn Future<Output = Result<R>> + Send + '_>> + Send,
        R: Send + 'static,
    {
        // Esta implementación es simplificada pero funcional para el ejemplo
        // En un entorno de producción, se necesitaría una implementación más robusta
        
        // Crear una nueva instancia temporal del repositorio para la transacción
        let repo = Box::new(Self {}) as Box<dyn UserRepositoryPort>;
        
        // Ejecutar la función en el contexto del nuevo repositorio
        f(repo).await
    }
}