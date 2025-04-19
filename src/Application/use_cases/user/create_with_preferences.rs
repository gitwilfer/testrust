// e:\work\cursos\anyB\src\Application\use_cases\user\create_with_preferences.rs

use std::sync::Arc;
use uuid::Uuid;
use anyhow::{Result, Context};
use async_trait::async_trait;
use chrono::Utc;
use log::{info, debug, error};

use crate::Application::dtos::create_user_dto::CreateUserDto;
use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::errors::application_error::ApplicationError;
use crate::Application::mappers::user_mapper::UserMapper;
use crate::Application::ports::driven::repositories::{UserQueryRepository, UserCommandRepository};
use crate::Application::ports::driven::AuthServicePort;
use crate::Application::ports::unit_of_work::{UnitOfWork, RepositoryRegistry};
use crate::Domain::entities::user::User;
use crate::Application::validators::user_validator::UserValidator;

// Estructura para preferencias de usuario (ejemplo)
#[derive(Clone, Debug)]
pub struct UserPreference {
    pub key: String,
    pub value: String,
}

// --- Trait del Caso de Uso (Definido localmente) ---
#[async_trait]
pub trait CreateUserWithPreferencesUseCase: Send + Sync {
     async fn execute(&self, user_dto: CreateUserDto, preferences: Vec<UserPreference>) -> Result<UserResponseDto, ApplicationError>;
}
// --------------------------------------------------

pub struct CreateUserWithPreferencesUseCaseImpl {
    user_command_repository: Arc<dyn UserCommandRepository>,
    user_query_repository: Arc<dyn UserQueryRepository>,
    auth_service: Arc<dyn AuthServicePort>,
    unit_of_work: Arc<dyn UnitOfWork>,
    user_mapper: Arc<UserMapper>,
    // preference_repository: Arc<dyn PreferenceRepository>,
}

impl CreateUserWithPreferencesUseCaseImpl {
    pub fn new(
        user_command_repository: Arc<dyn UserCommandRepository>,
        user_query_repository: Arc<dyn UserQueryRepository>,
        auth_service: Arc<dyn AuthServicePort>,
        unit_of_work: Arc<dyn UnitOfWork>,
        user_mapper: Arc<UserMapper>,
        // preference_repository: Arc<dyn PreferenceRepository>,
    ) -> Self {
        CreateUserWithPreferencesUseCaseImpl {
            user_command_repository,
            user_query_repository,
            auth_service,
            unit_of_work,
            user_mapper,
            // preference_repository,
        }
    }

    async fn validate_unique_fields(&self, username: &str, email: &str) -> Result<(), ApplicationError> {
        // ... (lógica de validación sin cambios) ...
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

    // Mover lógica principal aquí o al impl del trait
    pub async fn execute(&self, user_dto: CreateUserDto, preferences: Vec<UserPreference>) -> Result<UserResponseDto, ApplicationError> {
        info!("Ejecutando caso de uso CreateUserWithPreferences: username='{}'", user_dto.username);

        // 1. Validar DTO
        if let Err(e) = UserValidator::validate_create_dto(&user_dto) { /* ... */ }
        debug!("DTO de creación validado.");

        // 2. Validar unicidad
        self.validate_unique_fields(&user_dto.username, &user_dto.email).await?;

        // 3. Hashear contraseña
        let hashed_password = self.auth_service.hash_password(&user_dto.password)
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error hashing password: {}", e)))?;
        debug!("Contraseña hasheada.");

        // 4. DTO a Entidad User
        let new_user_entity = self.user_mapper.to_entity(user_dto, hashed_password)
            .map_err(|e| ApplicationError::MappingError(format!("Error mapping DTO to entity: {}", e)))?;
        debug!("Entidad User creada desde DTO.");

        // 5. Guardar usuario y preferencias DENTRO de UoW
        info!("Iniciando Unidad de Trabajo para crear usuario con preferencias...");
        let created_user = self.unit_of_work.execute(|registry| {
            let user_to_create = new_user_entity.clone();
            let prefs_to_create = preferences.clone();
            async move {
                debug!("Dentro de UoW: Obteniendo repositorios y conexión...");
                let user_cmd_repo = registry.user_command_repository();
                // let pref_cmd_repo = registry.preference_command_repository();
                let conn = registry.get_diesel_async_conn();

                // Crear usuario
                debug!("Dentro de UoW: Llamando a user_cmd_repo.create...");
                let created_user_in_tx = user_cmd_repo.create(conn, user_to_create).await.context("...")?;
                debug!("Dentro de UoW: Usuario creado en BD con ID: {}", created_user_in_tx.id);

                // Crear preferencias
                if !prefs_to_create.is_empty() { /* ... lógica placeholder ... */ }

                Ok(created_user_in_tx)
            }
        }).await.map_err(|e| ApplicationError::UnitOfWorkError(format!("Unit of Work execution failed: {}", e)))?;
        info!("Unidad de Trabajo completada. Usuario creado con ID: {}", created_user.id);

        // 6. Mapear a DTO respuesta
        Ok(self.user_mapper.to_dto(created_user))
    }
}

// --- Implementar el trait LOCAL ---
#[async_trait]
// --- CORREGIDO: Eliminar la ruta explícita ---
impl CreateUserWithPreferencesUseCase for CreateUserWithPreferencesUseCaseImpl {
// -------------------------------------------
     async fn execute(&self, user_dto: CreateUserDto, preferences: Vec<UserPreference>) -> Result<UserResponseDto, ApplicationError> {
         // Llamar a la lógica principal (si se movió al impl de la struct)
         self.execute(user_dto, preferences).await
         // O mover toda la lógica aquí si se prefiere
     }
}
