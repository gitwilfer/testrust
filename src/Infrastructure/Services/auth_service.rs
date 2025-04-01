use async_trait::async_trait;
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, errors::Error};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use std::env;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn generate_token(&self, user_id: String) -> Result<String, Error>;
    async fn validate_token(&self, token: String) -> Result<bool, Error>;
}

#[derive(Clone)]
pub struct JwtAuthService;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[async_trait]
impl AuthService for JwtAuthService {
    async fn generate_token(&self, user_id: String) -> Result<String, Error> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let expiration = Utc::now() + Duration::hours(1);

        let claims = Claims {
            sub: user_id,
            exp: expiration.timestamp() as usize,
        };

        let header = Header::default();
        let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))?;

        Ok(token)
    }

    async fn validate_token(&self, token: String) -> Result<bool, Error> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let validation = Validation::default();
        let _token_data = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation)?;
        Ok(true)
    }
}
