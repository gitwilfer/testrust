use crate::application::dtos::user_dto::UserResponseDto;
use crate::application::errors::application_error::ApplicationError;
use crate::application::mappers::user_mapper::UserMapper;
use crate::domain::repositories::user_repository::UserRepository;
use std::sync::Arc;

pub struct FindUserByUsernameUseCase<R: UserRepository> {
    pub user_repository: Arc<R>,
    pub user_mapper: Arc<UserMapper>,
}

impl<R: UserRepository + Send + Sync + 'static> FindUserByUsernameUseCase<R> {
    pub fn new(
        user_repository: Arc<R>,
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
        let user = self.user_repository
            .find_by_username(username)
            .await
            .map_err(|e| ApplicationError::InternalError(format!("Error al buscar usuario: {}", e)))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Usuario con username '{}' no encontrado", username)))?;

        // 3. Mapear a DTO y devolver
        Ok(self.user_mapper.to_dto(user))
    }
}
