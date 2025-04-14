use crate::Application::dtos::update_user_dto::UpdateUserDto;
use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::errors::application_error::ApplicationError;
use crate::Application::mappers::user_mapper::UserMapper;
use crate::Application::ports::repositories::{UserRepositoryPort, UserQueryRepository};
use crate::Application::ports::repositories::AuthServicePort;
use crate::Application::validators::user_validator::UserValidator;
use chrono::Utc;
use uuid::Uuid;
use std::sync::Arc;
use async_trait::async_trait;
use log::{debug, error, warn};

pub struct UpdateUserUseCase {
    pub user_repository: Arc<dyn UserRepositoryPort>,
    pub user_query_repository: Arc<dyn UserQueryRepository>,
    pub user_mapper: Arc<UserMapper>,
    pub auth_service: Arc<dyn AuthServicePort>,
}

impl UpdateUserUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepositoryPort>,
        user_query_repository: Arc<dyn UserQueryRepository>,
        user_mapper: Arc<UserMapper>,
        auth_service: Arc<dyn AuthServicePort>,
    ) -> Self {
        UpdateUserUseCase {
            user_repository,
            user_query_repository,
            user_mapper,
            auth_service,
        }
    }

    pub async fn execute(&self, id: Uuid, update_dto: UpdateUserDto, modified_by: Option<Uuid>) -> Result<UserResponseDto, ApplicationError> {
        debug!("Iniciando caso de uso para actualizar usuario ID: {}", id);
        
        // 1. Validar campos
        if let Err(e) = UserValidator::validate_update_dto(&update_dto) {
            error!("Error de validación en la actualización de usuario: {}", e);
            return Err(ApplicationError::ValidationError(e.to_string()));
        }
        
        // 2. Verificar que el usuario existe usando CQRS (repositorio de consulta)
        debug!("Buscando usuario por ID: {}", id);
        let user_exists = self.user_query_repository
            .find_by_id(id)
            .await
            .map_err(|e| {
                error!("Error al buscar usuario por ID {}: {}", id, e);
                ApplicationError::InfrastructureError(format!("Error al buscar usuario: {}", e))
            })?;
            
        let mut user = match user_exists {
            Some(user) => {
                debug!("Usuario encontrado: {}", user.id);
                user
            },
            None => {
                warn!("Usuario con ID {} no encontrado", id);
                return Err(ApplicationError::NotFound(format!("Usuario con ID {} no encontrado", id)));
            }
        };
        
        // 3. Validar email único si se está actualizando
        if let Some(email) = &update_dto.email {
            debug!("Verificando unicidad de email: {}", email);
            if let Some(existing_user) = self.user_query_repository.find_by_email(email).await
                .map_err(|e| {
                    error!("Error al buscar usuario por email {}: {}", email, e);
                    ApplicationError::InfrastructureError(format!("Error al buscar usuario por email: {}", e))
                })?
            {
                if existing_user.id != id {
                    warn!("Email '{}' ya está registrado por otro usuario", email);
                    return Err(ApplicationError::Conflict(format!("El email '{}' ya está registrado por otro usuario", email)));
                }
            }
        }
        
        // 4. Aplicar actualizaciones básicas usando el mapper
        debug!("Aplicando actualizaciones al usuario");
        self.user_mapper.apply_updates(&mut user, &update_dto);
        
        // 5. Actualizar contraseña si se proporcionó
        if let Some(password) = &update_dto.password {
            debug!("Actualizando contraseña del usuario");
            user.password = self.auth_service
                .hash_password(password)
                .map_err(|e| {
                    error!("Error al hashear la contraseña: {}", e);
                    ApplicationError::InfrastructureError(format!("Error al hashear la contraseña: {}", e))
                })?;
        }
        
        // 6. Actualizar metadatos
        user.modified_by = modified_by;
        user.modified_at = Some(Utc::now().naive_utc());
        
        // 7. Guardar en repositorio
        debug!("Guardando usuario actualizado en base de datos");
        let updated_user = self.user_repository
            .update(user)
            .await
            .map_err(|e| {
                error!("Error al actualizar usuario {}: {}", id, e);
                ApplicationError::InfrastructureError(format!("Error al actualizar el usuario: {}", e))
            })?;
        
        // 8. Mapear a DTO de respuesta
        debug!("Usuario actualizado correctamente: {}", updated_user.id);
        Ok(self.user_mapper.to_dto(updated_user))
    }
}

#[async_trait]
impl crate::Application::use_cases::traits::UpdateUserUseCase for UpdateUserUseCase {
    async fn execute(&self, id: Uuid, update_dto: UpdateUserDto, modified_by: Option<Uuid>) -> Result<UserResponseDto, ApplicationError> {
        self.execute(id, update_dto, modified_by).await
    }
}