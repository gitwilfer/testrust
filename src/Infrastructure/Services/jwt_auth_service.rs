use anyhow::Error;
use uuid::Uuid;
use crate::Domain::services::AuthService;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Clone)]
pub struct JwtAuthService;

impl JwtAuthService {
    pub fn new() -> Self {
        JwtAuthService
    }
}

impl AuthService for JwtAuthService {
    fn hash_password(&self, password: &str) -> Result<String, Error> {
        Ok(hash(password, DEFAULT_COST)?)
    }

    fn verify_password(&self, hash: &str, password: &str) -> Result<bool, Error> {
        Ok(verify(password, hash)?)
    }

    fn generate_token(&self, _user_id: Uuid) -> Result<String, Error> { // Prefijado con _
        // Implementación temporal
        Ok("token".to_string())
    }

    fn verify_token(&self, _token: &str) -> Result<Uuid, Error> { // Prefijado con _
        // Implementación temporal
        Ok(Uuid::new_v4())
    }
}
