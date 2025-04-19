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
use anyhow::Context; // Añadir Context
use log::info; // Añadir log

// --- Trait del Caso de Uso (si existe en traits/find_all.rs) ---
#[async_trait]
pub trait FindAllUsersUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<UserResponseDto>, ApplicationError>;
}
// ------------------------------------------------------------

// Renombrar struct a Impl
pub struct FindAllUsersUseCaseImpl {
    // --- Dependencia Correcta ---
    user_query_repository: Arc<dyn UserQueryRepository>,
    // ----------------------------
    user_mapper: Arc<UserMapper>,
}

impl FindAllUsersUseCaseImpl {
    pub fn new(
        // --- Argumento Correcto ---
        user_query_repository: Arc<dyn UserQueryRepository>,
        // ----------------------------
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        // Renombrar struct a Impl
        FindAllUsersUseCaseImpl {
            user_query_repository,
            user_mapper,
        }
    }

    // Mover lógica principal aquí
    pub async fn execute(&self) -> Result<Vec<UserResponseDto>, ApplicationError> {
        info!("Ejecutando caso de uso FindAllUsers");
        // Obtener todos los usuarios del repositorio de CONSULTA
        let users = self.user_query_repository // <-- Usar el repo correcto
            .find_all()
            .await
            .context("Error al obtener todos los usuarios") // Usar anyhow::Context
            .map_err(|e| ApplicationError::InfrastructureError(e.to_string()))?; // Mapear a ApplicationError

        info!("Encontrados {} usuarios", users.len());
        // Mapear cada usuario a un DTO usando el mapper
        let user_dtos = users
            .into_iter()
            .map(|user| self.user_mapper.to_dto(user))
            .collect();

        // Devolver la lista de DTOs
        Ok(user_dtos)
    }
}

// Implementar el trait (si existe)
#[async_trait]
impl crate::Application::use_cases::traits::FindAllUsersUseCase for FindAllUsersUseCaseImpl {
    async fn execute(&self) -> Result<Vec<UserResponseDto>, ApplicationError> {
        self.execute().await
    }
}
