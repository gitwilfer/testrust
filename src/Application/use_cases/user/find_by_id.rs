use crate::application::dtos::user_dto::UserResponseDto;
use crate::application::errors::application_error::ApplicationError;
use crate::application::mappers::user_mapper::UserMapper;
use crate::domain::repositories::user_repository::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct FindUserByIdUseCase<R: UserRepository> {
    pub user_repository: Arc<R>,
    pub user_mapper: Arc<UserMapper>,
}

impl<R: UserRepository + Send + Sync + 'static> FindUserByIdUseCase<R> {
    pub fn new(
        user_repository: Arc<R>,
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        FindUserByIdUseCase {
            user_repository,
            user_mapper,
        }
    }

    pub async fn execute(&self, id: Uuid) -> Result<UserResponseDto, ApplicationError> {
        // 1. Buscar usuario por ID
        let user = self.user_repository
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::InternalError(format!("Error al buscar usuario: {}", e)))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Usuario con ID {} no encontrado", id)))?;

        // 2. Mapear a DTO y devolver
        Ok(self.user_mapper.to_dto(user))
    }
}
