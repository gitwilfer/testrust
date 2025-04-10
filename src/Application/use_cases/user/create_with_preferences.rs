use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;

use crate::Application::dtos::create_user_dto::CreateUserDto;
use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::errors::application_error::ApplicationError;
use crate::Application::mappers::user_mapper::UserMapper;
use crate::Application::ports::repositories::{UserRepositoryPort, UserQueryRepository, AuthServicePort};
use crate::Domain::entities::user::User;

// Estructura para preferencias de usuario (ejemplo)
pub struct UserPreference {
    pub key: String,
    pub value: String,
}

// Caso de uso para crear un usuario con preferencias
pub struct CreateUserWithPreferencesUseCase {
    // Cambiamos de UnitOfWork a usar directamente el repositorio
    user_repository: Arc<dyn UserRepositoryPort>,
    user_query_repository: Arc<dyn UserQueryRepository>,
    auth_service: Arc<dyn AuthServicePort>,
    user_mapper: Arc<UserMapper>,
}

impl CreateUserWithPreferencesUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepositoryPort>,
        user_query_repository: Arc<dyn UserQueryRepository>,
        auth_service: Arc<dyn AuthServicePort>,
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        Self {
            user_repository,
            user_query_repository,
            auth_service,
            user_mapper,
        }
    }

    async fn validate_unique_fields(&self, username: &str, email: &str) -> Result<(), ApplicationError> {
        // Primero verificamos si el username ya existe
        if let Some(_) = self.user_query_repository.find_by_username(username).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario por username: {}", e)))? {
            return Err(ApplicationError::Conflict(format!("El username '{}' ya está registrado", username)));
        }

        // Luego verificamos si el email ya existe
        if let Some(_) = self.user_query_repository.find_by_email(email).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario por email: {}", e)))? {
            return Err(ApplicationError::Conflict(format!("El email '{}' ya está registrado", email)));
        }

        Ok(())
    }

    pub async fn execute(&self, user_dto: CreateUserDto, preferences: Vec<UserPreference>) -> Result<UserResponseDto, ApplicationError> {
        // 1. Validar campos usando el validador
        if let Err(e) = crate::Application::validators::user_validator::UserValidator::validate_create_dto(&user_dto) {
            return Err(ApplicationError::ValidationError(e.to_string()));
        }

        // 2. Validar campos únicos (usando la parte Query)
        self.validate_unique_fields(&user_dto.username, &user_dto.email).await?;

        // 3. Hashear contraseña
        let hashed_password = self.auth_service
            .hash_password(&user_dto.password)
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al hashear la contraseña: {}", e)))?;

        // 4. Crear entidad User directamente
        let new_user = User {
            id: Uuid::new_v4(),
            username: user_dto.username,
            first_name: user_dto.first_name,
            last_name: user_dto.last_name,
            email: user_dto.email,
            password: hashed_password,
            created_by: None,
            created_at: Utc::now().naive_utc(),
            modified_by: None,
            modified_at: None,
            status: 1, // Active por defecto
        };

        // 5. Crear usuario usando el repositorio directamente
        // En una implementación completa con Unit of Work, este sería el lugar para usar la transacción
        let created_user = self.user_repository.create(new_user).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al crear usuario: {}", e)))?;

        // 6. En un sistema real, aquí crearías las preferencias
        // Por ahora solo mostramos que recibimos las preferencias
        if !preferences.is_empty() {
            log::info!("Se procesarían {} preferencias para el usuario {}", 
                      preferences.len(), created_user.id);
            // En una implementación real: self.preference_repository.create_many(...)
        }

        // 7. Mapear a DTO de respuesta
        Ok(self.user_mapper.to_dto(created_user))
    }
}