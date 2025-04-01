use crate::application::dtos::user_dto::UserResponseDto;
use crate::application::errors::application_error::ApplicationError;
use crate::application::mappers::user_mapper::UserMapper;
use crate::application::ports::repositories::UserRepositoryPort;
use std::sync::Arc;
use uuid::Uuid;

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
        // Buscar usuario por ID dentro de una transacción
        let user = self.user_repository
            .transaction(|tx| {
                Box::pin(async move {
                    tx.find_by_id(id).await
                })
            })
            .await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario: {}", e)))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Usuario con ID {} no encontrado", id)))?;
    
        // Mapear a DTO y devolver
        Ok(self.user_mapper.to_dto(user))
    }
}