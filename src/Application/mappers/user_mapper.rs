use crate::Domain::entities::user::User;
use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::dtos::create_user_dto::CreateUserDto;
use crate::Application::dtos::update_user_dto::UpdateUserDto;
use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;

pub struct UserMapper {}

impl UserMapper {
    pub fn new() -> Self {
        UserMapper {}
    }

    // Mapeo de Entity a DTO (domain -> application)
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
            status: entity.status as i32,
        }
    }

    // Mapeo de CreateUserDto a Entity (application -> domain)
    pub fn to_entity(&self, dto: CreateUserDto, hashed_password: String) -> Result<User, anyhow::Error> {
        Ok(User {
            id: Uuid::new_v4(),
            username: dto.username,
            first_name: dto.first_name,
            last_name: dto.last_name,
            email: dto.email,
            password: hashed_password,
            created_by: None,
            created_at: Utc::now().naive_utc(),
            modified_by: None,
            modified_at: None,
            status: 1, // Active por defecto
        })
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