use crate::Application::dtos::update_user_dto::UpdateUserDto;
use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::errors::application_error::ApplicationError;
use crate::Application::mappers::user_mapper::UserMapper;
// --- Traits Correctos ---
use crate::Application::ports::driven::repositories::{UserQueryRepository, UserCommandRepository};
use crate::Application::ports::driven::AuthServicePort;
// --- UoW ---
use crate::Application::ports::unit_of_work::{UnitOfWork, RepositoryRegistry};
// --------------------
use crate::Application::validators::user_validator::UserValidator;
use crate::Domain::entities::user::User; // Importar entidad
use chrono::Utc;
use uuid::Uuid;
use std::sync::Arc;
use async_trait::async_trait;
use log::{debug, error, warn, info}; // Añadir logs
use anyhow::{Context, anyhow}; // Añadir anyhow y Context

// --- Trait del Caso de Uso (si existe en traits/update.rs) ---
#[async_trait]
pub trait UpdateUserUseCase: Send + Sync {
    async fn execute(&self, id: Uuid, update_dto: UpdateUserDto, updated_by: Option<Uuid>) -> Result<UserResponseDto, ApplicationError>;
}
// -----------------------------------------------------------

// Renombrar struct a Impl
pub struct UpdateUserUseCaseImpl {
    // --- Dependencias Correctas ---
    user_command_repository: Arc<dyn UserCommandRepository>,
    user_query_repository: Arc<dyn UserQueryRepository>,
    auth_service: Arc<dyn AuthServicePort>,
    unit_of_work: Arc<dyn UnitOfWork>, // Añadido
    // ----------------------------
    user_mapper: Arc<UserMapper>,
}

impl UpdateUserUseCaseImpl {
    pub fn new(
        // --- Argumentos Correctos ---
        user_command_repository: Arc<dyn UserCommandRepository>,
        user_query_repository: Arc<dyn UserQueryRepository>,
        auth_service: Arc<dyn AuthServicePort>,
        unit_of_work: Arc<dyn UnitOfWork>, // Añadido
        // ----------------------------
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        // Renombrar struct a Impl
        UpdateUserUseCaseImpl {
            user_command_repository,
            user_query_repository,
            auth_service,
            unit_of_work, // Guardar UoW
            user_mapper,
        }
    }

    // Mover lógica principal aquí
    pub async fn execute(&self, id: Uuid, update_dto: UpdateUserDto, updated_by: Option<Uuid>) -> Result<UserResponseDto, ApplicationError> {
        debug!("Iniciando caso de uso para actualizar usuario ID: {}", id);

        // 1. Validar campos DTO
        if let Err(e) = UserValidator::validate_update_dto(&update_dto) {
            error!("Error de validación en la actualización de usuario: {}", e);
            return Err(ApplicationError::ValidationError(e.to_string()));
        }
        debug!("DTO de actualización validado.");

        // 2. Verificar que el usuario existe (usando Query Repo)
        debug!("Buscando usuario por ID: {}", id);
        let mut user = self.user_query_repository
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario: {}", e)))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Usuario con ID {} no encontrado", id)))?;
        debug!("Usuario encontrado: {}", user.id);

        // 3. Validar email único si se está actualizando (usando Query Repo)
        if let Some(email) = &update_dto.email {
            debug!("Verificando unicidad de email: {}", email);
            if let Some(existing_user) = self.user_query_repository.find_by_email(email).await
                .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario por email: {}", e)))?
            {
                if existing_user.id != id {
                    warn!("Email '{}' ya está registrado por otro usuario", email);
                    return Err(ApplicationError::Conflict(format!("El email '{}' ya está registrado por otro usuario", email)));
                }
            }
            debug!("Email es único o pertenece al mismo usuario.");
        }

        // 4. Aplicar actualizaciones básicas usando el mapper
        debug!("Aplicando actualizaciones al usuario");
        self.user_mapper.apply_updates(&mut user, &update_dto);

        // 5. Actualizar contraseña si se proporcionó
        let mut password_changed = false;
        if let Some(password) = &update_dto.password {
            if !password.is_empty() { // Solo hashear si no está vacío
                debug!("Actualizando contraseña del usuario");
                user.password = self.auth_service
                    .hash_password(password)
                    .map_err(|e| ApplicationError::InfrastructureError(format!("Error al hashear la contraseña: {}", e)))?;
                password_changed = true;
            }
        }
        if password_changed { debug!("Contraseña actualizada y hasheada."); }

        // 6. Actualizar metadatos
        user.updated_by = updated_by;
        user.updated_at = Some(Utc::now());
        debug!("Metadatos de actualización aplicados.");

        // 7. Guardar en repositorio DENTRO de UoW
        info!("Iniciando Unidad de Trabajo para actualizar usuario...");
        let updated_user_entity = self.unit_of_work.execute(|registry| {
            // Clonar la entidad actualizada para moverla al closure async
            let user_to_update = user.clone();
            async move {
                debug!("Dentro de UoW: Obteniendo repositorio de comando y conexión...");
                let cmd_repo = registry.user_command_repository();
                let conn = registry.get_diesel_async_conn();

                debug!("Dentro de UoW: Llamando a cmd_repo.update...");
                // Llamar al método update del repo de comando, pasando la conexión
                let result = cmd_repo.update(conn, user_to_update).await
                    .context("Failed to update user within Unit of Work")?; // Usar anyhow::Result

                debug!("Dentro de UoW: Usuario actualizado en BD con ID: {}", result.id);
                Ok(result) // Devolver la entidad User actualizada
            }
        }).await.map_err(|e| {
            error!("Error durante la Unidad de Trabajo al actualizar usuario {}: {:?}", id, e);
            ApplicationError::InfrastructureError(format!("Error en transacción al actualizar usuario: {}", e))
        })?;
        info!("Unidad de Trabajo completada. Usuario actualizado con ID: {}", updated_user_entity.id);

        // 8. Mapear a DTO de respuesta
        Ok(self.user_mapper.to_dto(updated_user_entity))
    }
}

// Implementar el trait (si existe)
#[async_trait]
impl crate::Application::use_cases::traits::UpdateUserUseCase for UpdateUserUseCaseImpl {
    async fn execute(&self, id: Uuid, update_dto: UpdateUserDto, updated_by: Option<Uuid>) -> Result<UserResponseDto, ApplicationError> {
        self.execute(id, update_dto, updated_by).await
    }
}
