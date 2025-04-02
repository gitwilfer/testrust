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

impl User {
    // Constructor para crear un nuevo usuario
    pub fn new(
        username: String,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
        created_by: Option<Uuid>,
    ) -> Self {
        use chrono::Utc;
        
        Self {
            id: Uuid::new_v4(),
            username,
            first_name,
            last_name,
            email,
            password,
            status: 1, // Activo por defecto
            created_at: Utc::now().naive_utc(),
            created_by,
            modified_at: None,
            modified_by: None,
        }
    }
    
    // Método para marcar como inactivo
    pub fn deactivate(&mut self, modified_by: Option<Uuid>) {
        use chrono::Utc;
        
        self.status = 0;
        self.modified_at = Some(Utc::now().naive_utc());
        self.modified_by = modified_by;
    }
    
    // Método para obtener nombre completo
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    
    // Método para verificar si está activo
    pub fn is_active(&self) -> bool {
        self.status == 1
    }
}