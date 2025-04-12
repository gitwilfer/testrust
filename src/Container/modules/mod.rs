pub mod user_module;
pub mod auth_module;
pub mod health_module;

use crate::container::builder::ContainerBuilder;
use anyhow::Result;
use log::info;

/// Registra todos los módulos estándar (sin optimización SQLx)
pub async fn register_all(builder: &mut ContainerBuilder) -> Result<()> {
    info!("Registrando todos los módulos...");
    
    user_module::register(builder)?;
    auth_module::register(builder)?;
    health_module::register(builder)?;
    
    info!("Todos los módulos registrados correctamente");
    Ok(())
}

/// Registra todos los módulos con optimización SQLx cuando es posible
pub async fn register_all_with_sqlx(builder: &mut ContainerBuilder) -> Result<()> {
    info!("Registrando todos los módulos con soporte SQLx...");
    
    user_module::register_with_sqlx(builder).await?;
    auth_module::register_with_sqlx(builder).await?;
    health_module::register(builder)?;
    
    info!("Todos los módulos registrados correctamente con soporte SQLx");
    Ok(())
}