pub mod dependency_provider;
pub mod implementations;

use anyhow::Result;
use std::sync::Arc;
use log::{info, warn};

use dependency_provider::{DependencyProvider, DefaultDependencyProvider};
use crate::Infrastructure::Persistence::sqlx_database;

// Re-exportar estructura principal para facilidad de uso
pub use dependency_provider::AppDependencies;

/// Crea las dependencias con implementaciones estándar basadas en Diesel
pub fn create_dependencies() -> Result<AppDependencies> {
    info!("Creando dependencias con implementaciones estándar");
    
    // Crear proveedor de dependencias
    let mut provider = DefaultDependencyProvider::new()?;
    
    // Registrar implementaciones de fábrica
    implementations::register_all(&mut provider)?;
    
    // Construir y retornar las dependencias
    let dependencies = provider.build();
    info!("Dependencias creadas correctamente");
    
    Ok(dependencies)
}

/// Crea las dependencias con soporte para SQLx en consultas
pub async fn create_dependencies_with_sqlx() -> Result<AppDependencies> {
    info!("Creando dependencias con soporte SQLx para consultas");
    
    // Intentar obtener pool de SQLx
    match sqlx_database::get_default_pool().await {
        Ok(sqlx_pool) => {
            let sqlx_pool_arc = Arc::new(sqlx_pool);
            
            // Crear proveedor de dependencias
            let mut provider = DefaultDependencyProvider::new()?;
            
            // Establecer pool de SQLx
            provider.set_sqlx_pool(sqlx_pool_arc);
            
            // Registrar implementaciones con soporte SQLx
            implementations::register_all_with_sqlx(&mut provider)?;
            
            // Construir y retornar las dependencias
            let dependencies = provider.build();
            info!("Dependencias creadas correctamente con soporte SQLx");
            
            Ok(dependencies)
        },
        Err(e) => {
            warn!("No se pudo inicializar con SQLx: {}. Usando implementaciones estándar", e);
            create_dependencies()
        }
    }
}