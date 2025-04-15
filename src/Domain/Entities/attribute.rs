use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Attribute {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub position: i16,
    pub is_unique_key: Option<i16>,
    pub data_type_id: Uuid,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub modified_by: Option<Uuid>,
    pub modified_at: Option<DateTime<Utc>>,
    pub status: i16,
}
