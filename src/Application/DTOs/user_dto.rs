use chrono::{Utc,DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<DateTime<Utc>>,
    pub status: i32,
}

// Eliminamos la implementación de Into que creaba dependencia circular