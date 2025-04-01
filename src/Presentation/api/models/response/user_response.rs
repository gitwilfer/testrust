use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Modelo para la respuesta de usuario
/// 
/// Este modelo representa la estructura de datos que se envía
/// como respuesta al cliente en operaciones relacionadas con usuarios.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
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