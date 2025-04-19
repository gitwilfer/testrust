use anyhow::Result;
use uuid::Uuid;
use async_trait::async_trait;
use diesel_async::AsyncPgConnection; // <-- AÑADIDO: Necesario para comandos
use crate::Domain::entities::user::User;

#[async_trait]
pub trait UserQueryRepository: Send + Sync {
    // Configurar la base de datos para consultas (opcional)
    fn set_database(&mut self, database_name: &str);

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn find_all(&self) -> Result<Vec<User>>;
}

/// Port para operaciones de comando (escritura) sobre la entidad User.
/// Estas operaciones deben ejecutarse dentro de una transacción (UoW)
/// y por lo tanto reciben la conexión transaccional asíncrona.
#[async_trait]
pub trait UserCommandRepository: Send + Sync {
    // --- AJUSTADO: Añadido 'conn' ---
    async fn create(&self, conn: &mut AsyncPgConnection, user: User) -> Result<User>;
    // --- AJUSTADO: Añadido 'conn' ---
    async fn update(&self, conn: &mut AsyncPgConnection, user: User) -> Result<User>;
    // --- AJUSTADO: Añadido 'conn' ---
    async fn delete(&self, conn: &mut AsyncPgConnection, id: Uuid) -> Result<()>;
}
