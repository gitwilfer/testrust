// src/application/use_cases/user/create_with_preferences.rs
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;

use crate::application::dtos::create_user_dto::CreateUserDto;
use crate::application::dtos::user_dto::UserResponseDto;
use crate::application::errors::application_error::ApplicationError;
use crate::application::mappers::user_mapper::UserMapper;
use crate::application::ports::repositories::{UserCommandRepository, UserQueryRepository};
use crate::application::ports::repositories::AuthServicePort;
use crate::application::ports::unit_of_work::{UnitOfWork, RepositoryRegistry};
use crate::application::validators::user_validator::UserValidator;

// Caso de uso que crea un usuario y sus preferencias en una sola transacción
pub struct CreateUserWithPreferencesUseCase {
    unit_of_work: Arc<dyn UnitOfWork>,
    user_query_repository: Arc<dyn UserQueryRepository>,
    auth_service: Arc<dyn AuthServicePort>,
    user_mapper: Arc<UserMapper>,
}

impl CreateUserWithPreferencesUseCase {
    pub fn new(
        unit_of_work: Arc<dyn UnitOfWork>,
        user_query_repository: Arc<dyn UserQueryRepository>,
        auth_service: Arc<dyn AuthServicePort>,
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        Self {
            unit_of_work,
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
        if let Err(e) = UserValidator::validate_create_dto(&user_dto) {
            return Err(ApplicationError::ValidationError(e.to_string()));
        }

        // 2. Validar campos únicos (usando la parte Query)
        self.validate_unique_fields(&user_dto.username, &user_dto.email).await?;

        // 3. Hashear contraseña
        let hashed_password = self.auth_service
            .hash_password(&user_dto.password)
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al hashear la contraseña: {}", e)))?;

        // 4. Crear entidad User
        let new_user = self.user_mapper.to_entity(user_dto, hashed_password)
            .map_err(|e| ApplicationError::ValidationError(format!("Error al crear entidad: {}", e)))?;

        // 5. Ejecutar transacción utilizando Unit of Work
        let result = self.unit_of_work.execute(|repos| {
            // Capturar las variables necesarias en closure
            let user_repository = repos.user_repository();
            let user = new_user.clone();
            let prefs = preferences.clone();
            
            Box::pin(async move {
                // 5.1 Crear usuario
                let created_user = user_repository.create(user).await?;
                
                // 5.2 Crear preferencias (esto sería otro repositorio en un escenario real)
                for pref in prefs {
                    // En un caso real, aquí llamaríamos a otro repositorio
                    // repos.preference_repository().create(pref, created_user.id).await?;
                }
                
                Ok(created_user)
            })
        }).await
        .map_err(|e| ApplicationError::InfrastructureError(format!("Error en transacción: {}", e)))?;

        // 6. Mapear a DTO de respuesta
        Ok(self.user_mapper.to_dto(result))
    }
}

// Estructura simple para el ejemplo
pub struct UserPreference {
    pub key: String,
    pub value: String,
}