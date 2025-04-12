pub mod builder;
pub mod app_state;
pub mod modules;

use anyhow::Result;
use log::{info, warn};

// Re-exportamos AppState para facilitar su uso
pub use app_state::AppState;

/// Función principal para construir el AppState con implementaciones estándar
pub async fn build() -> Result<AppState> {
    info!("Inicializando contenedor con implementaciones estándar");
    let mut builder = builder::create_builder();
    
    // Registrar todos los módulos con implementaciones estándar
    modules::register_all(&mut builder).await?;
    
    // Construir AppState
    let app_state = builder.build()?;
    info!("Contenedor inicializado correctamente");
    
    Ok(app_state)
}

/// Función para construir el AppState con soporte SQLx para consultas
pub async fn build_with_sqlx() -> Result<AppState> {
    info!("Inicializando contenedor con soporte SQLx para consultas");
    let mut builder = builder::create_builder();
    
    // Intentar inicializar con SQLx
    match modules::register_all_with_sqlx(&mut builder).await {
        Ok(_) => {
            info!("Contenedor inicializado con soporte SQLx para consultas");
        },
        Err(e) => {
            warn!("No se pudo inicializar con SQLx: {}. Usando implementación Diesel estándar", e);
            modules::register_all(&mut builder).await?;
        }
    }
    
    // Construir AppState
    let app_state = builder.build()?;
    Ok(app_state)
}