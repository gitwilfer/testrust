use actix_web::{web, HttpResponse, get, Error};
use std::sync::Arc;
use crate::Container::AppState; // Importar AppState

use crate::Infrastructure::monitoring::database_health_monitor::DatabaseHealthMonitor;

pub struct HealthController {
    db_monitor: Arc<DatabaseHealthMonitor>,
}

impl HealthController {
    pub fn new(db_monitor: Arc<DatabaseHealthMonitor>) -> Self {
        Self {
            db_monitor,
        }
    }
}

#[get("")]
async fn health_check(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> { // Cambiar a AppState
    // Acceder al controlador específico desde AppState
    let db_health = app_state.health_controller_data.db_monitor.get_health_data().await;
    let all_healthy = app_state.health_controller_data.db_monitor.all_healthy().await;
    
    let time_since_check = app_state.health_controller_data.db_monitor.time_since_last_check().await
        .map(|d| d.as_secs())
        .unwrap_or(0);
    
    let response = serde_json::json!({
        "status": if all_healthy { "healthy" } else { "degraded" },
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "last_check_seconds_ago": time_since_check,
        "database_health": db_health,
    });
    
    if all_healthy {
        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::ServiceUnavailable().json(response))
    }
}

// Configuración de las rutas
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(health_check)
    );
}