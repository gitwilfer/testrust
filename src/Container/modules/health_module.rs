use std::sync::Arc;
use anyhow::Result;
use log::{info, debug};

use crate::Container::builder::ContainerBuilder;
use crate::Infrastructure::monitoring::DatabaseHealthMonitor;
use crate::Presentation::api::controllers::HealthController;

/// Registra todos los componentes relacionados con el monitoreo de salud del sistema
pub fn register(builder: &mut ContainerBuilder) -> Result<()> {
    debug!("Registrando componentes del módulo de monitoreo");
    
    // Crear monitor de salud de base de datos
    // Verificar cada 60 segundos
    let db_monitor = Arc::new(DatabaseHealthMonitor::new(60));
    
    // Iniciar monitoreo en segundo plano
    db_monitor.start_monitoring();
    builder.register_service(db_monitor.clone());
    debug!("Monitor de salud de base de datos registrado e iniciado");
    
    // Registrar controlador de salud
    let health_controller = HealthController::new(db_monitor);
    builder.register_arc_service(Arc::new(health_controller));
    debug!("Controlador de salud registrado");
    
    info!("Módulo de monitoreo registrado correctamente");
    Ok(())
}

// No es necesario una versión específica para SQLx ya que el monitoreo
// funciona igual con ambas opciones