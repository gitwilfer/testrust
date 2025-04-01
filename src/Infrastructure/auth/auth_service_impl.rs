// src/infrastructure/auth/auth_service_impl.rs
use crate::domain::services::AuthService;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::env;
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct AuthServiceImpl {
    jwt_secret: String,
}

impl AuthServiceImpl {
    pub fn new() -> Self {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Self { jwt_secret }
    }
}

impl AuthService for AuthServiceImpl {
    fn hash_password(&self, password: &str) -> Result<String, anyhow::Error> {
        let hashed = hash(password, DEFAULT_COST)?;
        Ok(hashed)
    }

    fn verify_password(&self, hash: &str, password: &str) -> Result<bool, anyhow::Error> {
        let result = verify(password, hash)?;
        Ok(result)
    }

    fn generate_token(&self, user_id: Uuid) -> Result<String, anyhow::Error> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }

    fn verify_token(&self, token: &str) -> Result<Uuid, anyhow::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        let user_id = Uuid::parse_str(&token_data.claims.sub)?;
        Ok(user_id)
    }
}