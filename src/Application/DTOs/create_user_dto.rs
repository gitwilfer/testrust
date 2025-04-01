use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 50, message = "El username debe tener entre 3 y 50 caracteres"))]
    pub username: String,
    
    #[validate(length(min = 1, max = 100, message = "El nombre debe tener entre 1 y 100 caracteres"))]
    pub first_name: String,
    
    #[validate(length(min = 1, max = 100, message = "El apellido debe tener entre 1 y 100 caracteres"))]
    pub last_name: String,
    
    #[validate(email(message = "El formato del email es inválido"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "La contraseña debe tener al menos 8 caracteres"))]
    pub password: String,
}

// Eliminamos la implementación de From que creaba dependencia circular