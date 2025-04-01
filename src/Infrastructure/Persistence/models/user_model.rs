use crate::domain::entities::user::User;
use chrono::NaiveDateTime;
use uuid::Uuid;
use validator::{Validate, ValidationError};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
    ).unwrap();
}

#[derive(Debug, Validate)]
pub struct UserModel {
    pub id: Uuid,
    
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,
    
    #[validate(length(max = 100, message = "First name must not exceed 100 characters"))]
    pub first_name: String,
    
    #[validate(length(max = 100, message = "Last name must not exceed 100 characters"))]
    pub last_name: String,
    
    #[validate(custom = "validate_email")]
    pub email: String,
    
    #[validate(custom = "validate_password")]
    pub password: String,
    
    pub created_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub modified_by: Option<Uuid>,
    pub modified_at: Option<NaiveDateTime>,
    pub status: i16,
}

fn validate_email(email: &str) -> Result<(), ValidationError> {
    if !EMAIL_REGEX.is_match(email) {
        return Err(ValidationError::new("invalid_email_format"));
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::new("password_needs_uppercase"));
    }
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(ValidationError::new("password_needs_number"));
    }
    Ok(())
}

impl UserModel {
    pub fn from_entity(user: User) -> Result<Self, ValidationError> {
        let model = UserModel {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: user.password,
            created_by: user.created_by,
            created_at: user.created_at,
            modified_by: user.modified_by,
            modified_at: user.modified_at,
            status: user.status,
        };
        
        model.validate()?;
        Ok(model)
    }

    pub fn to_entity(&self) -> User {
        User {
            id: self.id,
            username: self.username.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
            created_by: self.created_by,
            created_at: self.created_at,
            modified_by: self.modified_by,
            modified_at: self.modified_at,
            status: self.status,
        }
    }
}
