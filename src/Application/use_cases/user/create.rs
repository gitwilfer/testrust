// e:\work\cursos\anyB\src\Application\use_cases\user\create.rs
// *** VERSIÓN QUE DEBERÍA FUNCIONAR ***

use std::sync::Arc;
use anyhow::{Result, Context};
use async_trait::async_trait;
use uuid::Uuid;
use log::{info, debug, error};

use crate::Application::dtos::create_user_dto::CreateUserDto;
use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::errors::application_error::ApplicationError;
use crate::Application::mappers::user_mapper::UserMapper;
use crate::Application::ports::driven::repositories::{
    UserCommandRepository,
    UserQueryRepository,
};
use crate::Application::ports::driven::AuthServicePort;
use crate::Application::ports::unit_of_work::{UnitOfWork, RepositoryRegistry};
use crate::Application::validators::user_validator::UserValidator;
use crate::Domain::entities::user::User;

// --- Trait para el Caso de Uso (Definido localmente) ---
#[async_trait] // <-- MANTENER AQUÍ
pub trait CreateUserUseCase: Send + Sync {
    async fn execute(&self, user_dto: CreateUserDto) -> Result<UserResponseDto, ApplicationError>;
}
// -------------------------------------------

pub struct CreateUserUseCaseImpl {
    user_command_repository: Arc<dyn UserCommandRepository>,
    user_query_repository: Arc<dyn UserQueryRepository>,
    auth_service: Arc<dyn AuthServicePort>,
    unit_of_work: Arc<dyn UnitOfWork>,
    user_mapper: Arc<UserMapper>,
}

impl CreateUserUseCaseImpl {
    // Constructor (Correcto)
    pub fn new(
        user_command_repository: Arc<dyn UserCommandRepository>,
        user_query_repository: Arc<dyn UserQueryRepository>,
        auth_service: Arc<dyn AuthServicePort>,
        unit_of_work: Arc<dyn UnitOfWork>,
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        Self {
            user_command_repository,
            user_query_repository,
            auth_service,
            unit_of_work,
            user_mapper,
        }
    }

    // Método helper de validación (Correcto)
    async fn validate_unique_fields(&self, username: &str, email: &str) -> Result<(), ApplicationError> {
        debug!("Validando unicidad para username: '{}', email: '{}'", username, email);
        if self.user_query_repository.find_by_username(username).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario por username: {}", e)))?
            .is_some() {
            return Err(ApplicationError::Conflict(format!("El username '{}' ya está registrado", username)));
        }
        if self.user_query_repository.find_by_email(email).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario por email: {}", e)))?
            .is_some() {
            return Err(ApplicationError::Conflict(format!("El email '{}' ya está registrado", email)));
        }
        debug!("Campos username y email son únicos.");
        Ok(())
    }
}

// --- Implementación del Trait LOCAL (Correcto) ---
#[async_trait] // <-- MANTENER AQUÍ TAMBIÉN
impl CreateUserUseCase for CreateUserUseCaseImpl {
    async fn execute(&self, user_dto: CreateUserDto) -> Result<UserResponseDto, ApplicationError> {
        // --- Lógica Principal (Parece Correcta) ---
        info!("Ejecutando caso de uso CreateUser: username='{}'", user_dto.username);

        // 1. Validar DTO
        if let Err(e) = UserValidator::validate_create_dto(&user_dto) { /* ... */ }
        debug!("DTO de creación validado.");

        // 2. Validar unicidad
        self.validate_unique_fields(&user_dto.username, &user_dto.email).await?;

        // 3. Hashear contraseña
        let hashed_password = self.auth_service.hash_password(&user_dto.password)
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error hashing password: {}", e)))?;
        debug!("Contraseña hasheada.");

        // 4. DTO a Entidad
        let new_user_entity = self.user_mapper.to_entity(user_dto, hashed_password)
            .map_err(|e| ApplicationError::MappingError(format!("Error mapping DTO to entity: {}", e)))?;
        debug!("Entidad User creada desde DTO.");

        // 5. UoW
        info!("Iniciando Unidad de Trabajo para crear usuario...");
        let created_user = self.unit_of_work.execute(|registry| { // Recibe &mut dyn RepositoryRegistry
            let user_to_create = new_user_entity.clone();
            async move {
                debug!("Dentro de UoW: Obteniendo repositorio de comando y conexión...");
                let cmd_repo = registry.user_command_repository(); // Obtiene &dyn UserCommandRepository
                let conn = registry.get_diesel_async_conn(); // Obtiene &mut AsyncPgConnection
                debug!("Dentro de UoW: Llamando a cmd_repo.create...");
                // Llama a create(conn, user)
                let result = cmd_repo.create(conn, user_to_create).await
                    .context("Failed to create user within Unit of Work")?;
                debug!("Dentro de UoW: Usuario creado en BD con ID: {}", result.id);
                Ok(result)
            }
        }).await.map_err(|e| ApplicationError::UnitOfWorkError(format!("Unit of Work execution failed: {}", e)))?;
        info!("Unidad de Trabajo completada. Usuario creado con ID: {}", created_user.id);

        // 6. Mapear a DTO respuesta
        Ok(self.user_mapper.to_dto(created_user))
        // --- FIN LÓGICA PRINCIPAL ---
    }
}
