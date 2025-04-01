use crate::application::errors::application_error::ApplicationError;
use crate::domain::repositories::user_repository::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteUserUseCase<R: UserRepository> {
    pub user_repository: Arc<R>,
}

impl<R: UserRepository + Send + Sync + 'static> DeleteUserUseCase<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        DeleteUserUseCase {
            user_repository,
        }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), ApplicationError> {
        // 1. Verificar que el usuario existe
        let user_exists = self.user_repository
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::InternalError(format!("Error al buscar usuario: {}", e)))?
            .is_some();

        if !user_exists {
            return Err(ApplicationError::NotFound(format!("Usuario con ID {} no encontrado", id)));
        }

        // 2. Eliminar usuario
        self.user_repository
            .transaction(|tx| {
                Box::pin(async move {
                    tx.delete(id).await
                })
            })
            .await
            .map_err(|e| ApplicationError::InternalError(format!("Error al eliminar el usuario: {}", e)))?;

        // 3. Devolver éxito
        Ok(())
    }
}
