use std::sync::Arc;
use anyhow::Result;
use log::{info, debug, trace};

use crate::factory::dependency_provider::DependencyProvider;
use crate::Infrastructure::monitoring::DatabaseHealthMonitor;

/// Registra todos los componentes relacionados con el monitoreo de salud
pub fn register(provider: &mut dyn DependencyProvider) -> Result<()> {
    debug!("Registrando componentes de monitoreo...");
    
    // Crear monitor de salud de base de datos
    // Verificar cada 60 segundos
    let db_monitor = Arc::new(DatabaseHealthMonitor::new(60));
    
    // Iniciar monitoreo en segundo plano
    db_monitor.start_monitoring();
    provider.register::<DatabaseHealthMonitor>(db_monitor);
    trace!("DatabaseHealthMonitor registrado");
    
    info!("Componentes de monitoreo registrados correctamente");
    Ok(())
}

// No es necesario una versión específica para SQLx ya que el monitoreo
// funciona igual con ambas opciones