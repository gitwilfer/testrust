use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(length(min = 1, message = "El nombre de usuario no puede estar vacío"))]
    pub username: String,
    
    #[validate(length(min = 1, message = "La contraseña no puede estar vacía"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDto {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user_id: uuid::Uuid,
}