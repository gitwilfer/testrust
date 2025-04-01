use chrono::NaiveDateTime;
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
    pub created_at: NaiveDateTime,
    pub modified_by: Option<Uuid>,
    pub modified_at: Option<NaiveDateTime>,
    pub status: i32,
}

// Implementación para facilitar la conversión a modelos de response
impl Into<crate::presentation::api::models::response::UserResponse> for UserResponseDto {
    fn into(self) -> crate::presentation::api::models::response::UserResponse {
        crate::presentation::api::models::response::UserResponse {
            id: self.id,
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            created_by: self.created_by,
            created_at: self.created_at,
            modified_by: self.modified_by,
            modified_at: self.modified_at,
            status: self.status,
        }
    }
}
