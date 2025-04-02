use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "El nombre de usuario no puede estar vacío"))]
    pub username: String,
    
    #[validate(length(min = 1, message = "La contraseña no puede estar vacía"))]
    pub password: String,
}