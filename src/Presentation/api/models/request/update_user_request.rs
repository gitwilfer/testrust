use serde::{Deserialize, Serialize};
use validator::Validate;

/// Modelo para la petición de actualización de usuario
/// 
/// Este modelo representa la estructura de datos que se recibe 
/// desde la API para actualizar un usuario existente.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 50, message = "El nombre debe tener entre 3 y 50 caracteres"))]
    pub first_name: Option<String>,
    
    #[validate(length(min = 3, max = 50, message = "El apellido debe tener entre 3 y 50 caracteres"))]
    pub last_name: Option<String>,
    
    #[validate(email(message = "El formato del email es inválido"))]
    pub email: Option<String>,
    
    #[validate(length(min = 8, message = "La contraseña debe tener al menos 8 caracteres"))]
    pub password: Option<String>,
}