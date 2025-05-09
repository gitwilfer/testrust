use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Tuple {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<DateTime<Utc>>,
    pub status: i16,
}
