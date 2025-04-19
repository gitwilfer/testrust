// Declaraciones de módulos existentes
pub mod builder;
pub mod app_state;
pub mod modules; // Contiene register_all

// Imports necesarios
use builder::ContainerBuilder;
use app_state::AppState;
use anyhow::Result;
use log::info;

// --- NUEVA FUNCIÓN UNIFICADA ---
/// Construye el AppState completo registrando todos los módulos.
/// Debe llamarse DESPUÉS de inicializar los pools de conexión.
pub async fn build_app_state() -> Result<AppState> {
    info!("Iniciando construcción del contenedor de dependencias...");
    let mut builder = ContainerBuilder::new();

    // Registrar todos los módulos usando la función unificada en modules/mod.rs
    // Esta función ahora configura todo (Diesel Async, SQLx, UoW, Repos, Casos de Uso, etc.)
    modules::register_all(&mut builder).await?;

    // Construir el AppState final a partir del registro poblado
    let app_state = builder.build()?; // Llama a build() en ContainerBuilder
    info!("Contenedor de dependencias construido exitosamente.");
    Ok(app_state)
}


