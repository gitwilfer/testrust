use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::persistence::database;
use crate::infrastructure::persistence::models::user_model::UserModel;
use crate::infrastructure::persistence::schema::user;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use std::error::Error;
use uuid::Uuid;

pub struct UserRepositoryImpl;

impl UserRepository for UserRepositoryImpl {
    fn create(&self, user: &User) -> Result<User, Box<dyn Error>> {
        let mut conn = database::get_main_connection()?;
        let user_model = UserModel {
            id: user.id,
            username: user.username.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
            status: user.status,
            created_at: user.created_at,
            created_by: user.created_by,
            modified_at: user.modified_at,
            modified_by: user.modified_by,
        };
        
        conn.transaction::<_, Box<dyn Error>, _>(|conn| {
            diesel::insert_into(schema::users::table)
                .values(&user_model)
                .execute(conn)?;
            
            Ok(())
        })?;
        
        Ok(user.clone())
    }
    
    fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
        let mut conn = database::get_main_connection()?;
        
        let result = schema::users::table
            .find(id)
            .first::<UserModel>(&mut conn)
            .optional()?;
        
        let user = result.map(|model| User {
            id: model.id,
            username: model.username,
            first_name: model.first_name,
            last_name: model.last_name,
            email: model.email,
            password: model.password,
            status: model.status,
            created_at: model.created_at,
            created_by: model.created_by,
            modified_at: model.modified_at,
            modified_by: model.modified_by,
        });
        
        Ok(user)
    }
    
    fn find_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn Error>> {
        let mut conn = database::get_main_connection()?;
        
        let result = schema::users::table
            .filter(schema::users::email.eq(email))
            .first::<UserModel>(&mut conn)
            .optional()?;
        
        let user = result.map(|model| User {
            id: model.id,
            username: model.username,
            first_name: model.first_name,
            last_name: model.last_name,
            email: model.email,
            password: model.password,
            status: model.status,
            created_at: model.created_at,
            created_by: model.created_by,
            modified_at: model.modified_at,
            modified_by: model.modified_by,
        });
        
        Ok(user)
    }
    
    fn update(&self, user: &User) -> Result<User, Box<dyn Error>> {
        let mut conn = database::get_main_connection()?;
        
        let user_model = UserModel {
            id: user.id,
            username: user.username.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
            status: user.status,
            created_at: user.created_at,
            created_by: user.created_by,
            modified_at: user.modified_at,
            modified_by: user.modified_by,
        };
        
        conn.transaction::<_, Box<dyn Error>, _>(|conn| {
            diesel::update(schema::users::table.find(user.id))
                .set(&user_model)
                .execute(conn)?;
            
            Ok(())
        })?;
        
        Ok(user.clone())
    }
    
    fn delete(&self, id: Uuid) -> Result<(), Box<dyn Error>> {
        let mut conn = database::get_main_connection()?;
        
        conn.transaction::<_, Box<dyn Error>, _>(|conn| {
            let affected = diesel::delete(schema::users::table.find(id))
                .execute(conn)?;
            
            if affected == 0 {
                return Err(Box::new(DieselError::NotFound) as Box<dyn Error>);
            }
            
            Ok(())
        })?;
        
        Ok(())
    }
    
    fn find_all(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let mut conn = database::get_main_connection()?;
        
        let models = schema::users::table
            .load::<UserModel>(&mut conn)?;
        
        let users = models
            .into_iter()
            .map(|model| User {
                id: model.id,
                username: model.username,
                first_name: model.first_name,
                last_name: model.last_name,
                email: model.email,
                password: model.password,
                status: model.status,
                created_at: model.created_at,
                created_by: model.created_by,
                modified_at: model.modified_at,
                modified_by: model.modified_by,
            })
            .collect();
        
        Ok(users)
    }
}
