use chrono::{DateTime, Utc, NaiveDateTime};
use uuid::Uuid;

use diesel::prelude::*;
use crate::Infrastructure::Persistence::schema::users;


// Quitar AsChangeset ya que está causando problemas con UUID
#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct UserModel {
    #[diesel(column_name = id)]
    pub id: Uuid,
    
    #[diesel(column_name = username)]
    pub username: String,
    
    #[diesel(column_name = first_name)]
    pub first_name: String,
    
    #[diesel(column_name = last_name)]
    pub last_name: String,
    
    #[diesel(column_name = email)]
    pub email: String,
    
    #[diesel(column_name = password_hash)]
    pub password: String,
    
    #[diesel(column_name = created_by)]
    pub created_by: Option<Uuid>,
    
    #[diesel(column_name = created_at)]
    #[diesel(skip_insertion)]
    pub created_at: NaiveDateTime,
    
    #[diesel(column_name = updated_by)]
    pub updated_by: Option<Uuid>,
    
    #[diesel(column_name = updated_at)]
    #[diesel(skip_insertion)]
    pub updated_at: Option<NaiveDateTime>, // Diesel mapea Timestamp a NaiveDateTime

    #[diesel(column_name = status)]
    pub status: i16, // i16 coincide con Int2
}

#[derive(AsChangeset, Debug)] // Solo AsChangeset (y Debug opcional)
#[diesel(table_name = users)]
pub struct UpdateUserChangeset {
    // Usa Option<> para permitir actualizaciones parciales si lo deseas
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    #[diesel(column_name = password_hash)] // Asegúrate que el nombre coincida
    pub password: Option<String>,
    pub updated_by: Option<Uuid>,
    // --- CAMBIO AQUÍ ---
    // Cambia DateTime<Utc> a NaiveDateTime para compatibilidad con AsChangeset
    pub updated_at: Option<chrono::NaiveDateTime>,
    // --- FIN DEL CAMBIO ---
    pub status: Option<i16>,
    // NO incluyas id, created_at, created_by
}