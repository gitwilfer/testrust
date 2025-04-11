use chrono::NaiveDateTime;
use uuid::Uuid;
use diesel::prelude::*;
use crate::Infrastructure::Persistence::schema::usuarios;

// Quitar AsChangeset ya que está causando problemas con UUID
#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = usuarios)]
pub struct UserModel {
    #[diesel(column_name = idx_usuario)]
    pub id: Uuid,
    
    #[diesel(column_name = usuario)]
    pub username: String,
    
    #[diesel(column_name = nombre)]
    pub first_name: String,
    
    #[diesel(column_name = apellido)]
    pub last_name: String,
    
    #[diesel(column_name = correo_electronico)]
    pub email: String,
    
    #[diesel(column_name = contraseña)]
    pub password: String,
    
    #[diesel(column_name = creado_por)]
    pub created_by: Option<Uuid>,
    
    #[diesel(column_name = fecha_creacion)]
    pub created_at: NaiveDateTime,
    
    #[diesel(column_name = modificado_por)]
    pub modified_by: Option<Uuid>,
    
    #[diesel(column_name = fecha_modificacion)]
    pub modified_at: Option<NaiveDateTime>,

    #[diesel(column_name = estado)]
    pub status: i16,
}