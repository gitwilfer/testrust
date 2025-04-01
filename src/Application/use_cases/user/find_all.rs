use crate::application::dtos::user_dto::UserResponseDto;
use crate::application::errors::application_error::ApplicationError;
use crate::application::mappers::user_mapper::UserMapper;
use crate::domain::repositories::user_repository::UserRepository;
use std::sync::Arc;

pub struct FindAllUsersUseCase<R: UserRepository> {
    pub user_repository: Arc<R>,
    pub user_mapper: Arc<UserMapper>,
}

impl<R: UserRepository + Send + Sync + 'static> FindAllUsersUseCase<R> {
    pub fn new(
        user_repository: Arc<R>,
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        FindAllUsersUseCase {
            user_repository,
            user_mapper,
        }
    }

    pub async fn execute(&self) -> Result<Vec<UserResponseDto>, ApplicationError> {
        // 1. Obtener todos los usuarios del repositorio
        let users = self.user_repository
            .find_all()
            .await
            .map_err(|e| ApplicationError::InternalError(format!("Error al obtener usuarios: {}", e)))?;

        // 2. Mapear cada usuario a un DTO
        let user_dtos = users
            .into_iter()
            .map(|user| self.user_mapper.to_dto(user))
            .collect();

        // 3. Devolver la lista de DTOs
        Ok(user_dtos)
    }
}
