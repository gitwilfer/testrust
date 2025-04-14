pub mod dependency_provider;
pub mod implementations;

use anyhow::Result;
use std::sync::Arc;
use log::{info, warn};

// Importar tanto el struct DefaultDependencyProvider como el trait DependencyProvider
use dependency_provider::{DefaultDependencyProvider, DependencyProvider};
use crate::Infrastructure::Persistence::sqlx_database;

/// Crea las dependencias con implementaciones estándar basadas en Diesel
pub fn create_dependencies() -> Result<dependency_provider::AppDependencies> {
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
pub async fn create_dependencies_with_sqlx() -> Result<dependency_provider::AppDependencies> {
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