use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

// Importar el schema específico para esta tabla
use crate::Infrastructure::Persistence::schema::logical_entities;

#[derive(Queryable, Selectable, Insertable, Debug, Clone)]
#[diesel(table_name = logical_entities)] // Usar el schema importado
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LogicalEntityModel {
    // Asumiendo que el ID es manejado por la base de datos (serial o default)
    // Si no, necesitarías manejarlo explícitamente o quitarlo de Insertable
    // y manejarlo en el repositorio. Por ahora, lo mantenemos como en el comentario.
    #[diesel(deserialize_as = Uuid)]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub assign_view: Option<String>,
    #[diesel(deserialize_as = Option<Uuid>)]
    pub created_by: Option<Uuid>,
    // Diesel async a menudo usa NaiveDateTime para timestamp sin zona horaria
    pub created_at: NaiveDateTime,
    #[diesel(deserialize_as = Option<Uuid>)]
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
    pub status: i16,
    // Añadir otros campos si existen en tu schema `logical_entities`
}

// Podrías añadir un `impl Default` o `new` si es útil
impl Default for LogicalEntityModel {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(), // O un valor default apropiado si no es generado por DB
            name: String::new(),
            description: None,
            assign_view: None,
            created_by: None,
            created_at: chrono::Utc::now().naive_utc(), // Default a ahora
            updated_by: None,
            updated_at: None,
            status: 1, // 1 un estado default apropiado
        }
    }
}