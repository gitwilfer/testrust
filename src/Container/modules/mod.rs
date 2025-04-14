// --- INICIO CÓDIGO CORREGIDO ---
pub mod user_module;
pub mod auth_module;
pub mod health_module;

use crate::Container::builder::ContainerBuilder;
use anyhow::Result;
use log::info;

/// Registra todos los módulos estándar (sin optimización SQLx)
pub async fn register_all(builder: &mut ContainerBuilder) -> Result<()> {
    info!("Registrando todos los módulos...");

    // Usar la sintaxis de función asociada: Modulo::Struct::funcion()
    user_module::UserModule::register(builder)?;
    auth_module::AuthModule::register(builder)?;
    health_module::HealthModule::register(builder)?;

    info!("Todos los módulos registrados correctamente");
    Ok(())
}

/// Registra todos los módulos con optimización SQLx cuando es posible
pub async fn register_all_with_sqlx(builder: &mut ContainerBuilder) -> Result<()> {
    info!("Registrando todos los módulos con soporte SQLx...");

    // Usar la sintaxis de función asociada: Modulo::Struct::funcion()
    user_module::UserModule::register_with_sqlx(builder).await?;
    auth_module::AuthModule::register_with_sqlx(builder).await?;
    // Health module no tiene versión SQLx, usamos la estándar
    health_module::HealthModule::register(builder)?;

    info!("Todos los módulos registrados correctamente con soporte SQLx");
    Ok(())
}
// --- FIN CÓDIGO CORREGIDO ---
