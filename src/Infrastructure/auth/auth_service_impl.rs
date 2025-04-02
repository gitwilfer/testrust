// src/infrastructure/auth/auth_service_impl.rs
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::application::ports::repositories::AuthServicePort;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,        // Subject (usuario ID)
    exp: usize,         // Tiempo de expiración
    iat: usize,         // Issued at (tiempo de emisión)
    role: Option<String>, // Role del usuario (opcional)
}

pub struct AuthServiceImpl {
    jwt_secret: String,
    token_expiration: u64, // Duración del token en segundos
}

impl AuthServiceImpl {
    pub fn new() -> Result<Self> {
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| anyhow!("JWT_SECRET debe estar configurada en las variables de entorno"))?;
        
        // Por defecto, tokens válidos por 24 horas
        let token_expiration = env::var("TOKEN_EXPIRATION_SECONDS")
            .unwrap_or_else(|_| "86400".to_string())
            .parse::<u64>()
            .unwrap_or(86400);
        
        Ok(Self { jwt_secret, token_expiration })
    }
}

#[async_trait]
impl AuthServicePort for AuthServiceImpl {
    fn hash_password(&self, password: &str) -> Result<String> {
        hash(password, DEFAULT_COST)
            .map_err(|e| anyhow!("Error al hashear la contraseña: {}", e))
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        verify(password, hash)
            .map_err(|e| anyhow!("Error al verificar la contraseña: {}", e))
    }

    async fn generate_token(&self, user_id: Uuid) -> Result<String> {
        // Obtener el tiempo actual en segundos desde el epoch
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        
        let claims = Claims {
            sub: user_id.to_string(),
            iat: now as usize,
            exp: (now + self.token_expiration) as usize,
            role: None, // Podríamos añadir roles en el futuro
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| anyhow!("Error al generar el token JWT: {}", e))
    }

    async fn validate_token(&self, token: &str) -> Result<Uuid> {
        // Decodificar y validar el token
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| anyhow!("Token JWT inválido: {}", e))?;
        
        // Extraer y convertir el ID de usuario
        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| anyhow!("ID de usuario inválido en el token"))?;
        
        Ok(user_id)
    }
}

// Tests unitarios para el servicio de autenticación
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_password_hash_and_verify() {
        // Configurar la variable de entorno para el test
        env::set_var("JWT_SECRET", "test_secret_key");
        
        let auth_service = AuthServiceImpl::new().unwrap();
        let password = "SecurePassword123!";
        
        // Probar hash
        let hashed = auth_service.hash_password(password).unwrap();
        assert_ne!(password, hashed, "El hash debe ser diferente a la contraseña original");
        
        // Probar verificación
        let is_valid = auth_service.verify_password(password, &hashed).unwrap();
        assert!(is_valid, "La verificación de la contraseña debería ser exitosa");
        
        // Probar verificación con contraseña incorrecta
        let is_invalid = auth_service.verify_password("WrongPassword", &hashed).unwrap();
        assert!(!is_invalid, "La verificación con contraseña incorrecta debería fallar");
    }

    #[tokio::test]
    async fn test_token_generation_and_validation() {
        // Configurar la variable de entorno para el test
        env::set_var("JWT_SECRET", "test_secret_key");
        
        let auth_service = AuthServiceImpl::new().unwrap();
        let user_id = Uuid::new_v4();
        
        // Generar token
        let token = auth_service.generate_token(user_id).await.unwrap();
        assert!(!token.is_empty(), "El token no debería estar vacío");
        
        // Validar token y verificar que devuelve el mismo ID
        let extracted_id = auth_service.validate_token(&token).await.unwrap();
        assert_eq!(user_id, extracted_id, "El ID extraído debería coincidir con el original");
    }
}