use async_trait::async_trait;
use anyhow::{Result, anyhow};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use std::sync::Arc;

use crate::application::ports::repositories::AuthServicePort;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,        // Subject (usuario ID)
    exp: usize,         // Tiempo de expiración
    iat: usize,         // Issued at (tiempo de emisión)
    role: Option<String>, // Role del usuario (opcional)
}

pub struct AuthServiceImpl {
    jwt_secret: Arc<String>,
    token_expiration: u64,
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
        
        Ok(Self { 
            jwt_secret: Arc::new(jwt_secret), 
            token_expiration 
        })
    }
}

// Implementamos Clone para permitir compartir el servicio entre hilos
impl Clone for AuthServiceImpl {
    fn clone(&self) -> Self {
        Self {
            jwt_secret: self.jwt_secret.clone(),
            token_expiration: self.token_expiration,
        }
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

        // Usamos Arc<String> para compartir la secret key entre hilos
        let jwt_secret = self.jwt_secret.clone();
        
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
        .map_err(|e| anyhow!("Error al generar el token JWT: {}", e))
    }

    async fn validate_token(&self, token: &str) -> Result<Uuid> {
        // Usamos Arc<String> para compartir la secret key entre hilos
        let jwt_secret = self.jwt_secret.clone();
        
        // Decodificar y validar el token
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| anyhow!("Token JWT inválido: {}", e))?;
        
        // Extraer y convertir el ID de usuario
        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| anyhow!("ID de usuario inválido en el token"))?;
        
        Ok(user_id)
    }
}