// Declarar los módulos restantes
pub mod user_module;
pub mod auth_module;
pub mod health_module;
pub mod database_module;
pub mod repository_module;
pub mod controller_module;

use crate::Container::builder::ContainerBuilder;
use anyhow::Result;
use log::info;

/// Registra todos los módulos de la aplicación.
/// Asume que la configuración (ej: pools de BD) ya está lista.
pub async fn register_all(builder: &mut ContainerBuilder) -> Result<()> {
    info!("Registrando todos los módulos...");

    // El orden puede ser importante si unos módulos dependen de otros
    // 1. Database (registra pools y UoW)
    database_module::register_database_dependencies(builder).await?;
    // 2. Repositories (registra repos de consulta SQLx)
    repository_module::register_repository_dependencies(builder).await?;
    // 3. Auth (registra AuthService y LoginUseCase, depende de UserQueryRepository)
    auth_module::AuthModule::register(builder)?;
    // 4. User (registra UserCommandRepo y casos de uso de User, depende de AuthService y UserQueryRepository)
    user_module::UserModule::register(builder)?;
    // 5. Controllers (dependen de Casos de Uso registrados por los módulos anteriores)
    controller_module::register_controller_dependencies(builder).await?;
    // 6. Health (depende de monitores, etc.)
    health_module::HealthModule::register(builder)?;


    info!("Todos los módulos registrados correctamente");
    Ok(())
}
