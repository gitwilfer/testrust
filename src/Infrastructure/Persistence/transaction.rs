use diesel::Connection;
use anyhow::Result;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use std::future::Future;
// use std::pin::Pin;
use log::debug;

pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

// Macro para ejecutar una función dentro de una transacción
// con manejo de errores consistente
#[macro_export]
macro_rules! transaction {
    ($conn:expr, $func:expr) => {
        $conn.transaction(|| {
            match $func($conn) {
                Ok(result) => Ok(result),
                Err(e) => {
                    error!("Error en transacción: {}", e);
                    Err(e)
                }
            }
        })
    };
}

// Función para ejecutar una operación asíncrona dentro de una transacción
pub async fn execute_async_transaction<F, Fut, R>(
    conn: &mut DbConnection,
    f: F,
) -> Result<R>
where
    F: FnOnce() -> Fut + Send,
    Fut: Future<Output = Result<R>> + Send,
    R: Send + 'static,
{
    debug!("Iniciando transacción asíncrona");
    
    // Ejecutar la transacción
    let result = conn.transaction(|conn|{
        // Tokio runtime para ejecutar la función asíncrona en el contexto de la transacción
        let runtime = tokio::runtime::Handle::current();
        runtime.block_on(async {
            f().await
        })
    });
    
    debug!("Transacción asíncrona completada");
    result
}

// Wrapper thread-safe alrededor de una transacción
pub struct Transaction<'a> {
    conn: &'a mut DbConnection,
}

impl<'a> Transaction<'a> {
    pub fn new(conn: &'a mut DbConnection) -> Self {
        Self { conn }
    }
    
    // Método para ejecutar operaciones SQL directamente
    pub fn execute<F, T>(&mut self, operation: F) -> Result<T>
    where
        F: FnOnce(&mut PgConnection) -> Result<T>,
    {
        operation(self.conn)
    }
}