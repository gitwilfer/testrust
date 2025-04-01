use crate::domain::entities::user::User;
// use crate::infrastructure::persistence::models::UserModel; // <-- Eliminado
use chrono::{Utc,DateTime,NaiveDateTime};
use uuid::Uuid;
use crate::application::dtos::user_dto::UserResponseDto;
use crate::application::dtos::update_user_dto::UpdateUserDto;
pub struct UserMapper {}

impl UserMapper {
    pub fn new() -> Self {
        UserMapper {}
    }

    pub fn to_dto(&self, entity: User) -> UserResponseDto {

        UserResponseDto  {
            id: entity.id,
            username: entity.username,
            first_name: entity.first_name,
            last_name: entity.last_name,
            email: entity.email,
            created_by: entity.created_by,
            created_at: entity.created_at,
            modified_by: entity.modified_by,
            modified_at: entity.modified_at,
            status: entity.status,
        }
    }

}
