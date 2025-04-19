use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc}; // O los tipos que devuelva tu caso de uso/DTO

#[derive(Serialize, Debug)]
pub struct LogicalEntityResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub assign_view: Option<String>,
    pub created_by: Option<Uuid>, // Asumiendo que el caso de uso devuelve esto
    pub created_at: DateTime<Utc>, // Asumiendo que el caso de uso devuelve esto
    pub updated_by: Option<Uuid>, // Asumiendo que el caso de uso devuelve esto
    pub updated_at: Option<DateTime<Utc>>, // Asumiendo que el caso de uso devuelve esto
    pub status: i16, // O el tipo que corresponda
}

// Una respuesta más simple solo para la creación, si prefieres
#[derive(Serialize, Debug)]
pub struct CreateLogicalEntityResponse {
    pub id: Uuid,
}
