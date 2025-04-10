use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::errors::application_error::ApplicationError;
use crate::Application::mappers::user_mapper::UserMapper;
use crate::Application::ports::repositories::UserRepositoryPort;
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;

pub struct FindUserByIdUseCase {
    pub user_repository: Arc<dyn UserRepositoryPort>,
    pub user_mapper: Arc<UserMapper>,
}

impl FindUserByIdUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepositoryPort>,
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        FindUserByIdUseCase {
            user_repository,
            user_mapper,
        }
    }

    pub async fn execute(&self, id: Uuid) -> Result<UserResponseDto, ApplicationError> {
        // Buscar usuario por ID
        let user = self.user_repository
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario: {}", e)))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Usuario con ID {} no encontrado", id)))?;
        
        // Mapear a DTO y devolver
        Ok(self.user_mapper.to_dto(user))
    }
}

#[async_trait]
impl crate::Application::use_cases::traits::FindUserByIdUseCase for FindUserByIdUseCase {
    async fn execute(&self, id: Uuid) -> Result<UserResponseDto, ApplicationError> {
        self.execute(id).await
    }
}