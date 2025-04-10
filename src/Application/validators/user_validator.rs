use regex::Regex;
use lazy_static::lazy_static;
use anyhow::{Result, anyhow};
use validator::Validate;
use crate::Application::dtos::create_user_dto::CreateUserDto;
use crate::Application::dtos::update_user_dto::UpdateUserDto;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\\-\\.]{1}[a-z0-9]+)*\\.[a-z]{2,6})"
    ).unwrap();
}

pub struct UserValidator;

impl UserValidator {
    // Validación manual para casos específicos
    pub fn validate_email(email: &str) -> Result<()> {
        if !EMAIL_REGEX.is_match(email) {
            return Err(anyhow!("Formato de email inválido"));
        }
        Ok(())
    }

    pub fn validate_username(username: &str) -> Result<()> {
        if username.len() < 3 {
            return Err(anyhow!("El username debe tener al menos 3 caracteres"));
        }
        if username.len() > 50 {
            return Err(anyhow!("El username no puede exceder 50 caracteres"));
        }
        Ok(())
    }

    pub fn validate_password(password: &str) -> Result<()> {
        if password.len() < 8 {
            return Err(anyhow!("La contraseña debe tener al menos 8 caracteres"));
        }
        if !password.chars().any(|c| c.is_uppercase()) {
            return Err(anyhow!("La contraseña debe contener al menos una mayúscula"));
        }
        if !password.chars().any(|c| c.is_numeric()) {
            return Err(anyhow!("La contraseña debe contener al menos un número"));
        }
        Ok(())
    }

    // Validación usando anotaciones
    pub fn validate_create_dto(dto: &CreateUserDto) -> Result<()> {
        if let Err(errors) = dto.validate() {
            return Err(anyhow!("Error de validación: {:?}", errors));
        }
        
        // Validaciones adicionales que no se pueden hacer con anotaciones
        Self::validate_password(&dto.password)?;
        
        Ok(())
    }

    pub fn validate_update_dto(dto: &UpdateUserDto) -> Result<()> {
        if let Err(errors) = dto.validate() {
            return Err(anyhow!("Error de validación: {:?}", errors));
        }
        
        // Validaciones adicionales para el password si está presente
        if let Some(password) = &dto.password {
            Self::validate_password(password)?;
        }
        
        Ok(())
    }
}
