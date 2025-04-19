// src/domain/entities/user.rs
use chrono::{NaiveDateTime, Utc, DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::{Result, anyhow};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\\-\\.]{1}[a-z0-9]+)*\\.[a-z]{2,6})"
    ).unwrap();
}

// Status como enum para mejor semántica
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Inactive = 0,
    Active = 1,
    Suspended = 2,
    PendingActivation = 3,
}

impl From<i16> for UserStatus {
    fn from(status: i16) -> Self {
        match status {
            0 => UserStatus::Inactive,
            1 => UserStatus::Active,
            2 => UserStatus::Suspended,
            3 => UserStatus::PendingActivation,
            _ => UserStatus::Inactive,
        }
    }
}

impl From<UserStatus> for i16 {
    fn from(status: UserStatus) -> Self {
        status as i16
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub status: i16,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>
}

impl User {
    // Constructor para crear un nuevo usuario con validaciones
    pub fn new(
        username: String,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
        created_by: Option<Uuid>,
    ) -> Result<Self> {
        // Validaciones inline
        if username.len() < 3 || username.len() > 50 {
            return Err(anyhow!("El nombre de usuario debe tener entre 3 y 50 caracteres"));
        }
        
        if first_name.is_empty() || first_name.len() > 100 {
            return Err(anyhow!("El nombre debe tener entre 1 y 100 caracteres"));
        }
        
        if last_name.is_empty() || last_name.len() > 100 {
            return Err(anyhow!("El apellido debe tener entre 1 y 100 caracteres"));
        }
        
        if !EMAIL_REGEX.is_match(&email) {
            return Err(anyhow!("El formato del email es inválido"));
        }
        
        // La contraseña se valida por separado porque puede estar hasheada
        if password.is_empty() {
            return Err(anyhow!("La contraseña no puede estar vacía"));
        }
        
        Ok(Self {
            id: Uuid::new_v4(),
            username,
            first_name,
            last_name,
            email,
            password,
            status: UserStatus::Active as i16, // Activo por defecto
            created_at: Utc::now(),
            created_by,
            updated_at: None,
            updated_by: None,
        })
    }
    
    // Método para activar el usuario
    pub fn activate(&mut self, updated_by: Option<Uuid>) -> Result<()> {
        if self.get_status() == UserStatus::Active {
            return Err(anyhow!("El usuario ya está activo"));
        }
        
        self.status = UserStatus::Active as i16;
        self.updated_at = Some(Utc::now());
        self.updated_by = updated_by;
        
        Ok(())
    }
    
    // Método para desactivar el usuario
    pub fn deactivate(&mut self, updated_by: Option<Uuid>) -> Result<()> {
        if self.get_status() == UserStatus::Inactive {
            return Err(anyhow!("El usuario ya está inactivo"));
        }
        
        self.status = UserStatus::Inactive as i16;
        self.updated_at = Some(Utc::now());
        self.updated_by = updated_by;
        
        Ok(())
    }
    
    // Método para suspender el usuario
    pub fn suspend(&mut self, updated_by: Option<Uuid>) -> Result<()> {
        if self.get_status() == UserStatus::Suspended {
            return Err(anyhow!("El usuario ya está suspendido"));
        }
        
        self.status = UserStatus::Suspended as i16;
        self.updated_at = Some(Utc::now());
        self.updated_by = updated_by;
        
        Ok(())
    }
    
    // Método para actualizar información del usuario
    pub fn update_info(
        &mut self, 
        first_name: Option<String>, 
        last_name: Option<String>, 
        email: Option<String>,
        updated_by: Option<Uuid>
    ) -> Result<()> {
        let mut changed = false;
        
        if let Some(first_name) = first_name {
            if first_name.is_empty() || first_name.len() > 100 {
                return Err(anyhow!("El nombre debe tener entre 1 y 100 caracteres"));
            }
            self.first_name = first_name;
            changed = true;
        }
        
        if let Some(last_name) = last_name {
            if last_name.is_empty() || last_name.len() > 100 {
                return Err(anyhow!("El apellido debe tener entre 1 y 100 caracteres"));
            }
            self.last_name = last_name;
            changed = true;
        }
        
        if let Some(email) = email {
            if !EMAIL_REGEX.is_match(&email) {
                return Err(anyhow!("El formato del email es inválido"));
            }
            self.email = email;
            changed = true;
        }
        
        if changed {
            self.updated_at = Some(Utc::now());
            self.updated_by = updated_by;
        }
        
        Ok(())
    }
    
    // Método para cambiar la contraseña (sin validación porque podría estar hasheada)
    pub fn set_password(&mut self, password: String, updated_by: Option<Uuid>) {
        self.password = password;
        self.updated_at = Some(Utc::now());
        self.updated_by = updated_by;
    }
    
    // Método para obtener nombre completo
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    
    // Método para verificar si está activo
    pub fn is_active(&self) -> bool {
        self.get_status() == UserStatus::Active
    }
    
    // Método para obtener el status como enum
    pub fn get_status(&self) -> UserStatus {
        UserStatus::from(self.status)
    }
}

// Tests unitarios para la entidad User
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_valid() {
        let result = User::new(
            "testuser".to_string(),
            "Test".to_string(),
            "User".to_string(),
            "test@example.com".to_string(),
            "SecurePassword123".to_string(),
            None,
        );
        
        assert!(result.is_ok(), "User creation should succeed with valid data");
        let user = result.unwrap();
        assert_eq!(user.username, "testuser");
        assert_eq!(user.status, 1);
        assert!(user.is_active());
    }

    #[test]
    fn test_user_creation_invalid_username() {
        let result = User::new(
            "te".to_string(), // Too short
            "Test".to_string(),
            "User".to_string(),
            "test@example.com".to_string(),
            "SecurePassword123".to_string(),
            None,
        );
        
        assert!(result.is_err(), "User creation should fail with short username");
    }

    #[test]
    fn test_user_creation_invalid_email() {
        let result = User::new(
            "testuser".to_string(),
            "Test".to_string(),
            "User".to_string(),
            "invalid-email".to_string(),
            "SecurePassword123".to_string(),
            None,
        );
        
        assert!(result.is_err(), "User creation should fail with invalid email");
    }

    #[test]
    fn test_user_deactivation() {
        let mut user = User::new(
            "testuser".to_string(),
            "Test".to_string(),
            "User".to_string(),
            "test@example.com".to_string(),
            "SecurePassword123".to_string(),
            None,
        ).unwrap();
        
        assert!(user.is_active());
        
        let result = user.deactivate(None);
        assert!(result.is_ok());
        assert!(!user.is_active());
        assert_eq!(user.status, 0);
    }

    #[test]
    fn test_user_update_info() {
        let mut user = User::new(
            "testuser".to_string(),
            "Test".to_string(),
            "User".to_string(),
            "test@example.com".to_string(),
            "SecurePassword123".to_string(),
            None,
        ).unwrap();
        
        let result = user.update_info(
            Some("Updated".to_string()),
            Some("Name".to_string()),
            Some("updated@example.com".to_string()),
            None,
        );
        
        assert!(result.is_ok());
        assert_eq!(user.first_name, "Updated");
        assert_eq!(user.last_name, "Name");
        assert_eq!(user.email, "updated@example.com");
        assert!(user.updated_at.is_some());
    }
}