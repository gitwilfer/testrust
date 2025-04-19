use crate::Application::errors::application_error::ApplicationError;
// --- Trait Correcto ---
use crate::Application::ports::driven::repositories::UserCommandRepository;
// --- UoW ---
use crate::Application::ports::unit_of_work::{UnitOfWork, RepositoryRegistry};
// --------------------
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use anyhow::{Result, Context}; // Usar Result de anyhow internamente
use log::{info, debug, error}; // Añadir logs

// --- Trait del Caso de Uso (si existe en traits/delete.rs) ---
#[async_trait]
pub trait DeleteUserUseCase: Send + Sync {
    async fn execute(&self, id: Uuid) -> Result<(), ApplicationError>;
}
// -----------------------------------------------------------

// Renombrar struct a Impl
pub struct DeleteUserUseCaseImpl {
    // --- Dependencias Correctas ---
    user_command_repository: Arc<dyn UserCommandRepository>,
    unit_of_work: Arc<dyn UnitOfWork>,
    // ----------------------------
}

impl DeleteUserUseCaseImpl {
    pub fn new(
        // --- Argumentos Correctos ---
        user_command_repository: Arc<dyn UserCommandRepository>,
        unit_of_work: Arc<dyn UnitOfWork>,
        // ----------------------------
    ) -> Self {
        // Renombrar struct a Impl
        DeleteUserUseCaseImpl {
            user_command_repository,
            unit_of_work,
        }
    }

    // Mover lógica principal aquí
    pub async fn execute(&self, id: Uuid) -> Result<(), ApplicationError> {
        info!("Ejecutando caso de uso DeleteUser: id='{}'", id);

        // --- Ejecutar dentro de UoW ---
        self.unit_of_work.execute(|registry| async move {
            debug!("Dentro de UoW para eliminar usuario: {}", id);
            let cmd_repo = registry.user_command_repository();
            let conn = registry.get_diesel_async_conn();

            // Llamar al método delete pasando la conexión.
            // El repo devuelve Result<()>, que incluye error si no se encuentra.
            cmd_repo.delete(conn, id).await
                .context(format!("Failed to delete user {} within Unit of Work", id))?; // Usar Result de anyhow

            debug!("Usuario eliminado en BD (dentro de UoW): {}", id);
            Ok(()) // Devolver Ok(()) si la operación fue exitosa dentro de la UoW
        })
        .await // Esperar a que la UoW termine
        .map_err(|e| { // Mapear error de UoW a ApplicationError
            error!("Error durante la Unidad de Trabajo al eliminar usuario {}: {:?}", id, e);
            // Intentar detectar si el error original fue "NotFound"
            if e.to_string().contains("no encontrado para eliminar") {
                 ApplicationError::NotFound(format!("Usuario con ID {} no encontrado para eliminar", id))
            } else {
                 ApplicationError::InfrastructureError(format!("Error en transacción al eliminar usuario: {}", e))
            }
        })?;
        // -----------------------------------------

        info!("Caso de uso DeleteUser completado exitosamente para ID: {}", id);
        Ok(())
    }
}

// Implementar el trait (si existe)
#[async_trait]
impl crate::Application::use_cases::traits::DeleteUserUseCase for DeleteUserUseCaseImpl {
    async fn execute(&self, id: Uuid) -> Result<(), ApplicationError> {
        self.execute(id).await
    }
}
