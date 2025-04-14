use std::sync::Arc;
use anyhow::Result;
use log::{info, debug};

use crate::Container::builder::ContainerBuilder;
use crate::Infrastructure::monitoring::DatabaseHealthMonitor;
use crate::Presentation::api::controllers::HealthController;

// --- Structs para devolver grupos de dependencias ---
struct HealthMonitor {
    db_monitor: Arc<DatabaseHealthMonitor>,
}
// --- Fin Structs ---

pub struct HealthModule;

impl HealthModule {
    // Solo se necesita una versión de register
    pub fn register(builder: &mut ContainerBuilder) -> Result<()> {
        debug!("Registrando componentes del módulo de monitoreo");
        let monitor = Self::build_and_register_monitor(builder)?;
        Self::build_and_register_controller(builder, monitor)?; // Pasar dependencias
        info!("Módulo de monitoreo registrado correctamente");
        Ok(())
    }

    // Helper para Monitor - Devuelve instancia
    fn build_and_register_monitor(builder: &mut ContainerBuilder) -> Result<HealthMonitor> {
        let db_monitor = Arc::new(DatabaseHealthMonitor::new(60));
        db_monitor.start_monitoring(); // Iniciar monitoreo
        builder.register_service(db_monitor.clone()); // Registrar el monitor
        debug!("Monitor de salud de base de datos registrado e iniciado");
        Ok(HealthMonitor { db_monitor }) // Devolver
    }

    // Helper para Controlador - Recibe instancia
    fn build_and_register_controller(
        builder: &mut ContainerBuilder,
        monitor: HealthMonitor, // Recibe monitor
    ) -> Result<()> {
        let health_controller = HealthController::new(monitor.db_monitor);
        // Usa register_arc_service si existe, sino register_service
        builder.register_arc_service(Arc::new(health_controller));
        // builder.register_service(Arc::new(health_controller)); // Alternativa

        debug!("Controlador de salud registrado");
        Ok(())
    }
}
