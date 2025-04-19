use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::errors::application_error::ApplicationError;
use crate::Application::mappers::user_mapper::UserMapper;
// --- Trait Correcto ---
use crate::Application::ports::driven::repositories::UserQueryRepository;
// --- ELIMINADO: UserRepositoryPort ---
// use crate::Application::ports::driven::repositories::UserRepositoryPort;
// --------------------
use std::sync::Arc;
use async_trait::async_trait; // Añadir si falta
use anyhow::{Context, anyhow}; // Añadir anyhow y Context
use log::info; // Añadir log

// --- Trait del Caso de Uso (si existe en traits/find_by_username.rs) ---
#[async_trait]
pub trait FindUserByUsernameUseCase: Send + Sync {
    async fn execute(&self, username: &str) -> Result<UserResponseDto, ApplicationError>;
}
// --------------------------------------------------------------------

// Renombrar struct a Impl
pub struct FindUserByUsernameUseCaseImpl {
    // --- Dependencia Correcta ---
    user_query_repository: Arc<dyn UserQueryRepository>,
    // ----------------------------
    user_mapper: Arc<UserMapper>,
}

impl FindUserByUsernameUseCaseImpl {
    pub fn new(
        // --- Argumento Correcto ---
        user_query_repository: Arc<dyn UserQueryRepository>,
        // ----------------------------
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        // Renombrar struct a Impl
        FindUserByUsernameUseCaseImpl {
            user_query_repository,
            user_mapper,
        }
    }

    // Mover lógica principal aquí
    pub async fn execute(&self, username: &str) -> Result<UserResponseDto, ApplicationError> {
        info!("Ejecutando caso de uso FindUserByUsername: username='{}'", username);
        // 1. Validar que el username no esté vacío
        if username.trim().is_empty() {
            return Err(ApplicationError::ValidationError("El username no puede estar vacío".to_string()));
        }

        // 2. Buscar usuario por username usando el repositorio de CONSULTA
        let user = self.user_query_repository // <-- Usar el repo correcto
            .find_by_username(username)
            .await
            .context(format!("Error al buscar usuario por username: {}", username)) // Usar anyhow::Context
            .map_err(|e| ApplicationError::InfrastructureError(e.to_string()))? // Mapear a ApplicationError
            .ok_or_else(|| ApplicationError::NotFound(format!("Usuario con username '{}' no encontrado", username)))?;

        info!("Usuario encontrado: {}", username);
        // 3. Mapear a DTO usando el mapper y devolver
        Ok(self.user_mapper.to_dto(user))
    }
}

// Implementar el trait (si existe)
#[async_trait]
impl crate::Application::use_cases::traits::FindUserByUsernameUseCase for FindUserByUsernameUseCaseImpl {
    async fn execute(&self, username: &str) -> Result<UserResponseDto, ApplicationError> {
        self.execute(username).await
    }
}
