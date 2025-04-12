pub mod user_factory;
pub mod auth_factory;
pub mod health_factory;

use anyhow::Result;
use log::{info, debug};

use crate::factory::dependency_provider::DependencyProvider;

/// Registra todas las implementaciones estándar
pub fn register_all(provider: &mut dyn DependencyProvider) -> Result<()> {
    info!("Registrando todas las implementaciones...");
    
    // Registrar componentes por dominio
    user_factory::register(provider)?;
    auth_factory::register(provider)?;
    health_factory::register(provider)?;
    
    info!("Todas las implementaciones registradas correctamente");
    Ok(())
}

/// Registra todas las implementaciones con soporte SQLx
pub fn register_all_with_sqlx(provider: &mut dyn DependencyProvider) -> Result<()> {
    info!("Registrando todas las implementaciones con soporte SQLx...");
    
    // Registrar componentes por dominio con soporte SQLx
    user_factory::register_with_sqlx(provider)?;
    auth_factory::register_with_sqlx(provider)?;
    health_factory::register(provider)?;
    
    info!("Todas las implementaciones con soporte SQLx registradas correctamente");
    Ok(())
}