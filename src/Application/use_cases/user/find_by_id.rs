use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::errors::application_error::ApplicationError;
use crate::Application::mappers::user_mapper::UserMapper;
use crate::Application::ports::driven::repositories::UserQueryRepository;
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use anyhow::{Context, anyhow}; // Añadir anyhow y Context
use log::info; // Añadir log

#[async_trait]
pub trait FindUserByIdUseCase: Send + Sync {
    async fn execute(&self, id: Uuid) -> Result<UserResponseDto, ApplicationError>;
}

pub struct FindUserByIdUseCaseImpl {
    // --- CORREGIDO: Usar UserQueryRepository ---
    user_query_repository: Arc<dyn UserQueryRepository>,
    // -----------------------------------------
    user_mapper: Arc<UserMapper>,
}

impl FindUserByIdUseCaseImpl {
    pub fn new(
        // --- CORREGIDO: Recibir UserQueryRepository ---
        user_query_repository: Arc<dyn UserQueryRepository>,
        // --------------------------------------------
        user_mapper: Arc<UserMapper>,
    ) -> Self {
        // Renombrar struct a Impl
        FindUserByIdUseCaseImpl {
            user_query_repository,
            user_mapper,
        }
    }

    // Mover la lógica principal aquí (o dejarla en el impl del trait)
    pub async fn execute(&self, id: Uuid) -> Result<UserResponseDto, ApplicationError> {
        info!("Ejecutando caso de uso FindUserById: id='{}'", id);
        // Buscar usuario por ID usando el repositorio de CONSULTA
        let user = self.user_query_repository // <-- Usar el repo correcto
            .find_by_id(id)
            .await
            // Usar anyhow::Context para errores internos
            .context(format!("Error al buscar usuario por ID: {}", id))
            // Mapear error de infraestructura a error de aplicación
            .map_err(|e| ApplicationError::InfrastructureError(e.to_string()))?
            // Mapear Option a Result (NotFound si es None)
            .ok_or_else(|| ApplicationError::NotFound(format!("Usuario con ID {} no encontrado", id)))?;

        info!("Usuario encontrado: {}", id);
        // Mapear a DTO y devolver
        Ok(self.user_mapper.to_dto(user))
    }
}

// Implementar el trait (si existe)
#[async_trait]
impl crate::Application::use_cases::traits::FindUserByIdUseCase for FindUserByIdUseCaseImpl {
    async fn execute(&self, id: Uuid) -> Result<UserResponseDto, ApplicationError> {
        // Llamar a la lógica principal
        self.execute(id).await
    }
}
