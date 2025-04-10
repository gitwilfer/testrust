use crate::Application::dtos::create_user_dto::CreateUserDto;
use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::errors::application_error::ApplicationError;
use crate::Application::mappers::user_mapper::UserMapper;
use crate::Application::ports::repositories::{UserRepositoryPort, AuthServicePort};
use crate::Application::validators::user_validator::UserValidator;
use std::sync::Arc;

pub struct CreateUserUseCase {
    user_repository: Arc<dyn UserRepositoryPort>,
    auth_service: Arc<dyn AuthServicePort>,
    user_mapper: Arc<UserMapper>,
}

impl CreateUserUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepositoryPort>,
        auth_service: Arc<dyn AuthServicePort>,
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        CreateUserUseCase {
            user_repository,
            auth_service,
            user_mapper,
        }
    }

    async fn validate_unique_fields(&self, username: &str, email: &str) -> Result<(), ApplicationError> {
        // Validar username único
        if let Some(_) = self.user_repository.find_by_username(username).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario por username: {}", e)))? {
            return Err(ApplicationError::Conflict(format!("El username '{}' ya está registrado", username)));
        }

        // Validar email único
        if let Some(_) = self.user_repository.find_by_email(email).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario por email: {}", e)))? {
            return Err(ApplicationError::Conflict(format!("El email '{}' ya está registrado", email)));
        }

        Ok(())
    }

    pub async fn execute(&self, user_dto: CreateUserDto) -> Result<UserResponseDto, ApplicationError> {
        // 1. Validar campos usando el validador
        if let Err(e) = UserValidator::validate_create_dto(&user_dto) {
            return Err(ApplicationError::ValidationError(e.to_string()));
        }

        // 2. Validar campos únicos
        self.validate_unique_fields(&user_dto.username, &user_dto.email).await?;

        // 3. Hashear contraseña
        let hashed_password = self.auth_service
            .hash_password(&user_dto.password)
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al hashear la contraseña: {}", e)))?;

        // 4. Convertir DTO a entidad using the mapper
        let new_user = self.user_mapper.to_entity(user_dto, hashed_password)
            .map_err(|e| ApplicationError::ValidationError(format!("Error al crear entidad de usuario: {}", e)))?;

        // 5. Guardar en repositorio dentro de una transacción
        let created_user = self.user_repository.create(new_user).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al crear el usuario: {}", e)))?;

        // 6. Mapear a DTO de respuesta
        Ok(self.user_mapper.to_dto(created_user))
    }
}

#[async_trait::async_trait]
impl crate::Application::use_cases::traits::CreateUserUseCase for CreateUserUseCase {
    async fn execute(&self, user_dto: CreateUserDto) -> Result<UserResponseDto, ApplicationError> {
        self.execute(user_dto).await
    }
}