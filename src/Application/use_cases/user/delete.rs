use crate::Application::errors::application_error::ApplicationError;
use crate::Application::ports::repositories::UserRepositoryPort;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteUserUseCase {
    pub user_repository: Arc<dyn UserRepositoryPort>,
}

impl DeleteUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepositoryPort>) -> Self {
        DeleteUserUseCase {
            user_repository,
        }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), ApplicationError> {
        // 1. Verificar que el usuario existe
        let user_exists = self.user_repository.find_by_id(id).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario: {}", e)))?
            .is_some();

        if !user_exists {
            return Err(ApplicationError::NotFound(format!("Usuario con ID {} no encontrado", id)));
        }

        // 2. Eliminar usuario
        self.user_repository.delete(id).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al eliminar el usuario: {}", e)))?;

        // 3. Devolver éxito
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::application::use_cases::traits::DeleteUserUseCase for DeleteUserUseCase {
    async fn execute(&self, id: Uuid) -> Result<(), ApplicationError> {
        self.execute(id).await
    }
}