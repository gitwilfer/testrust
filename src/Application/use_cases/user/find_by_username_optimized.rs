// src/Application/use_cases/user/find_by_username_optimized.rs

use async_trait::async_trait;
use std::sync::Arc;

use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::errors::application_error::ApplicationError;
use crate::Application::mappers::user_mapper::UserMapper;
use crate::Application::ports::repositories::UserQueryRepository;
use crate::Application::use_cases::traits::FindUserByUsernameUseCase;

// ⭐ Caso de uso optimizado que utiliza directamente el repositorio de consulta
pub struct FindUserByUsernameOptimizedUseCase {
    pub user_query_repository: Arc<dyn UserQueryRepository>,
    pub user_mapper: Arc<UserMapper>,
}

impl FindUserByUsernameOptimizedUseCase {
    pub fn new(
        user_query_repository: Arc<dyn UserQueryRepository>,
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        Self {
            user_query_repository,
            user_mapper,
        }
    }

    pub async fn execute(&self, username: &str) -> Result<UserResponseDto, ApplicationError> {
        // Validar que el username no esté vacío
        if username.trim().is_empty() {
            return Err(ApplicationError::ValidationError("El username no puede estar vacío".to_string()));
        }
    
        // ⭐ Usar directamente el repositorio de consulta (SQLx)
        let user = self.user_query_repository.find_by_username(username).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario: {}", e)))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Usuario con username '{}' no encontrado", username)))?;
    
        // Mapear a DTO usando el mapper y devolver
        Ok(self.user_mapper.to_dto(user))
    }
}

#[async_trait]
impl FindUserByUsernameUseCase for FindUserByUsernameOptimizedUseCase {
    async fn execute(&self, username: &str) -> Result<UserResponseDto, ApplicationError> {
        self.execute(username).await
    }
}