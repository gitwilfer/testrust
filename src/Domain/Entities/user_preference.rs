use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserPreference {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub key: String,
    pub value: String,
}
