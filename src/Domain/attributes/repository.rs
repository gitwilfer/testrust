use async_trait::async_trait;
use super::attribute::Attribute;
use crate::Domain::error::DomainError;
use uuid::Uuid;
// Importar el tipo de conexión de Diesel si es necesario pasarlo
// use diesel::pg::PgConnection;

#[async_trait]
pub trait AttributeRepository: Send + Sync {
    // Método para guardar un lote de atributos DENTRO de una transacción existente.
    // Necesita la conexión/transacción, el ID de la entidad padre,
    // el ID del usuario creador y la lista de atributos a guardar.
    // NOTA: Este método NO debe manejar la transacción (begin/commit/rollback),
    // eso se hace en la capa de Aplicación.
    // Usamos un método sync aquí si se integra con Diesel sync.
    // Si todo es async, mantener async fn. Ajustar según el ORM.

    // Ejemplo SÍNCRONO (para Diesel síncrono):
    fn save_batch_sync(
        &self,
        // conn: &mut PgConnection, // Pasar la conexión transaccional
        entity_id: Uuid,
        user_id: Uuid,
        attributes: &[Attribute],
    ) -> Result<(), DomainError>;

    // Alternativa ASÍNCRONA (si se usa un ORM async o wrapper):
    // async fn save_batch(
    //     &self,
    //     entity_id: Uuid,
    //     user_id: Uuid,
    //     attributes: &[Attribute],
    // ) -> Result<(), DomainError>;
}
