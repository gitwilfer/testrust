use crate::application::dtos::update_user_dto::UpdateUserDto;
use crate::application::dtos::user_dto::UserResponseDto;
use crate::application::errors::application_error::ApplicationError;
use crate::application::mappers::user_mapper::UserMapper;
use crate::application::ports::repositories::{UserRepositoryPort, AuthServicePort};
use crate::application::validators::user_validator::UserValidator;
use chrono::Utc;
use uuid::Uuid;
use std::sync::Arc;

pub struct UpdateUserUseCase {
    pub user_repository: Arc<dyn UserRepositoryPort>,
    pub user_mapper: Arc<UserMapper>,
    pub auth_service: Arc<dyn AuthServicePort>,
}

impl UpdateUserUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepositoryPort>,
        user_mapper: Arc<UserMapper>,
        auth_service: Arc<dyn AuthServicePort>,
    ) -> Self {
        UpdateUserUseCase {
            user_repository,
            user_mapper,
            auth_service,
        }
    }

    async fn validate_unique_email(&self, id: Uuid, email: &str) -> Result<(), ApplicationError> {
        let existing_user = self.user_repository
            .find_by_email(email)
            .await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario por email: {}", e)))?;
            
        if let Some(user) = existing_user {
            if user.id != id {
                return Err(ApplicationError::Conflict(format!("El email '{}' ya está registrado por otro usuario", email)));
            }
        }
        Ok(())
    }

    pub async fn execute(&self, id: Uuid, update_dto: UpdateUserDto, modified_by: Option<Uuid>) -> Result<UserResponseDto, ApplicationError> {
        // 1. Validar campos
        if let Err(e) = UserValidator::validate_update_dto(&update_dto) {
            return Err(ApplicationError::ValidationError(e.to_string()));
        }

        // 2. Verificar que el usuario existe
        let mut user = self.user_repository
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario: {}", e)))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Usuario con ID {} no encontrado", id)))?;

        // 3. Validar email único si se está actualizando
        if let Some(email) = &update_dto.email {
            self.validate_unique_email(id, email).await?;
        }

        // 4. Aplicar actualizaciones
        if let Some(first_name) = update_dto.first_name {
            user.first_name = first_name;
        }
        if let Some(last_name) = update_dto.last_name {
            user.last_name = last_name;
        }
        if let Some(email) = update_dto.email {
            user.email = email;
        }

        // 5. Actualizar contraseña si se proporcionó
        if let Some(password) = &update_dto.password {
            user.password = self.auth_service
                .hash_password(password)
                .map_err(|e| ApplicationError::InfrastructureError(format!("Error al hashear la contraseña: {}", e)))?;
        }

        // 6. Actualizar metadatos
        user.modified_by = modified_by;
        user.modified_at = Some(Utc::now().naive_utc());

        // 7. Guardar en repositorio
        let updated_user = self.user_repository
            .transaction(|tx| {
                Box::pin(async move {
                    tx.update(user).await
                })
            })
            .await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al actualizar el usuario: {}", e)))?;

        // 8. Mapear a DTO de respuesta
        Ok(self.user_mapper.to_dto(updated_user))
    }
}