use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::errors::application_error::ApplicationError;
use crate::Application::mappers::user_mapper::UserMapper;
use crate::Application::ports::repositories::UserRepositoryPort;
use std::sync::Arc;

pub struct FindUserByUsernameUseCase {
    pub user_repository: Arc<dyn UserRepositoryPort>,
    pub user_mapper: Arc<UserMapper>,
}

impl FindUserByUsernameUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepositoryPort>,
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        FindUserByUsernameUseCase {
            user_repository,
            user_mapper,
        }
    }

    pub async fn execute(&self, username: &str) -> Result<UserResponseDto, ApplicationError> {
        // 1. Validar que el username no esté vacío
        if username.trim().is_empty() {
            return Err(ApplicationError::ValidationError("El username no puede estar vacío".to_string()));
        }
    
        // 2. Buscar usuario por username
        let username_clone = username.to_string(); // Clonar para mover dentro del closure
        let user = self.user_repository.find_by_username(&username_clone).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario: {}", e)))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Usuario con username '{}' no encontrado", username)))?;
    
        // 3. Mapear a DTO usando el mapper y devolver
        Ok(self.user_mapper.to_dto(user))
    }
}

#[async_trait::async_trait]
impl crate::Application::use_cases::traits::FindUserByUsernameUseCase for FindUserByUsernameUseCase {
    async fn execute(&self, username: &str) -> Result<UserResponseDto, ApplicationError> {
        self.execute(username).await
    }
}