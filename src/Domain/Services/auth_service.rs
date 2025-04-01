use uuid::Uuid;

pub trait AuthService {
    fn hash_password(&self, password: &str) -> Result<String, anyhow::Error>;
    fn verify_password(&self, hash: &str, password: &str) -> Result<bool, anyhow::Error>;
    fn generate_token(&self, user_id: Uuid) -> Result<String, anyhow::Error>;
    fn verify_token(&self, token: &str) -> Result<Uuid, anyhow::Error>;
}
