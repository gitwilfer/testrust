use crate::domain::entities::user::User;
use crate::application::dtos::user_dto::UserResponseDto;
use crate::application::dtos::update_user_dto::UpdateUserDto;

pub struct UserMapper {}

impl UserMapper {
    pub fn new() -> Self {
        UserMapper {}
    }

    pub fn to_dto(&self, entity: User) -> UserResponseDto {
        UserResponseDto {
            id: entity.id,
            username: entity.username,
            first_name: entity.first_name,
            last_name: entity.last_name,
            email: entity.email,
            created_by: entity.created_by,
            created_at: entity.created_at,
            modified_by: entity.modified_by,
            modified_at: entity.modified_at,
            status: entity.status as i32, // Convertir i16 a i32
        }
    }

    // Método para aplicar actualizaciones de un DTO a una entidad
    pub fn apply_updates(&self, user: &mut User, update_dto: &UpdateUserDto) {
        if let Some(first_name) = &update_dto.first_name {
            user.first_name = first_name.clone();
        }
        if let Some(last_name) = &update_dto.last_name {
            user.last_name = last_name.clone();
        }
        if let Some(email) = &update_dto.email {
            user.email = email.clone();
        }
        // La contraseña se maneja por separado debido al hashing
    }
}