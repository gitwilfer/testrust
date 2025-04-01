use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub status: i16,
    pub created_at: NaiveDateTime,
    pub created_by: Option<Uuid>,
    pub modified_at: Option<NaiveDateTime>,
    pub modified_by: Option<Uuid>
}
