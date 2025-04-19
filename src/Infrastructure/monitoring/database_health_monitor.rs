use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::interval;
use log::{info, warn, error, debug};
use std::collections::HashMap;

use crate::Infrastructure::Persistence::connection_pools;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseHealth {
    pub name: String,
    pub healthy: bool,
    //#[serde(with = "chrono::serde::ts_rfc3339")]
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub response_time_ms: u64,
}

pub struct DatabaseHealthMonitor {
    health_data: Arc<Mutex<Vec<DatabaseHealth>>>,
    last_check: Arc<Mutex<Option<Instant>>>,
    check_interval: Duration,
}

impl DatabaseHealthMonitor {
    pub fn new(check_interval_seconds: u64) -> Self {
        Self {
            health_data: Arc::new(Mutex::new(Vec::new())),
            last_check: Arc::new(Mutex::new(None)),
            check_interval: Duration::from_secs(check_interval_seconds),
        }
    }
    
    // Iniciar monitoreo en segundo plano
    pub fn start_monitoring(&self) {
        let health_data = self.health_data.clone();
        let last_check = self.last_check.clone();
        let check_interval = self.check_interval;
        
        tokio::spawn(async move {
            let mut interval_timer = interval(check_interval);
            
            loop {
                interval_timer.tick().await;
                
                // Capturamos el tiempo de inicio antes de cualquier operación
                let start = Instant::now();
                let now = chrono::Utc::now();
                
                // Verificar salud de Diesel de manera segura
                let combined_health = match connection_pools::check_pools_health().await {
                    Ok(health) => health,
                    Err(e) => {
                        error!("Error al verificar salud de todos los pools: {}", e);
                        HashMap::new() // Devuelve vacío si falla la verificación general
                    }
                };
                
                // Preparar nuevos datos de salud
                let mut health_entries = Vec::new();
                
                // Procesar resultados Diesel
                for (full_name, healthy) in combined_health {
                    // Calcular tiempo aquí puede ser menos preciso si hubo muchos checks
                    let response_time = start.elapsed().as_millis() as u64;
                    health_entries.push(DatabaseHealth {
                        name: full_name.clone(), // El nombre ya incluye prefijo (ej: "sqlx_main")
                        healthy,
                        last_check: now,
                        response_time_ms: response_time,
                    });

                    if !healthy {
                        warn!("Pool {} no está saludable", full_name);
                    }
                }
                
                // Actualizar datos compartidos
                if let Ok(mut data) = health_data.lock() {
                    *data = health_entries;
                } else {
                    error!("No se pudo obtener lock para actualizar health_data");
                }
                
                if let Ok(mut check_time) = last_check.lock() {
                    *check_time = Some(start);
                } else {
                    error!("No se pudo obtener lock para actualizar last_check");
                }
                
                info!("Verificación de salud de bases de datos completada");
            }
        });
    }
    
    // Métodos para acceder a los datos (no cambian)
    pub async fn get_health_data(&self) -> Vec<DatabaseHealth> {
        match self.health_data.lock() {
            Ok(data) => data.clone(),
            Err(_) => {
                error!("No se pudo obtener lock para leer health_data");
                Vec::new()
            },
        }
    }
    
    pub async fn all_healthy(&self) -> bool {
        match self.health_data.lock() {
            Ok(data) => data.iter().all(|h| h.healthy),
            Err(_) => {
                error!("No se pudo obtener lock para verificar health_data");
                false
            },
        }
    }
    
    pub async fn time_since_last_check(&self) -> Option<Duration> {
        match self.last_check.lock() {
            Ok(last) => last.map(|t| t.elapsed()),
            Err(_) => {
                error!("No se pudo obtener lock para leer last_check");
                None
            },
        }
    }
}