// src/Infrastructure/monitoring/database_health_monitor.rs

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex; // Cambiamos a Mutex para simplificar
use tokio::time::interval;
use log::{info, warn, error};
use std::collections::HashMap;

use crate::Infrastructure::Persistence::database;
use crate::Infrastructure::Persistence::sqlx_database;

#[derive(Debug, Clone)]
pub struct DatabaseHealth {
    pub name: String,
    pub healthy: bool,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub response_time_ms: u64,
}

// Datos que serán compartidos entre hilos
struct SharedState {
    health_data: Vec<DatabaseHealth>,
    last_check: Option<Instant>,
}

pub struct DatabaseHealthMonitor {
    shared: Arc<Mutex<SharedState>>,
    check_interval: Duration,
}

impl DatabaseHealthMonitor {
    pub fn new(check_interval_seconds: u64) -> Self {
        Self {
            shared: Arc::new(Mutex::new(SharedState {
                health_data: Vec::new(),
                last_check: None,
            })),
            check_interval: Duration::from_secs(check_interval_seconds),
        }
    }
    
    // Iniciar monitoreo en segundo plano
    pub fn start_monitoring(&self) {
        // Clonar el Arc que contiene el estado compartido
        let shared = self.shared.clone();
        let check_interval = self.check_interval;
        
        // Spawn una tarea en el runtime de tokio
        tokio::spawn(async move {
            let mut interval_timer = interval(check_interval);
            
            loop {
                interval_timer.tick().await;
                
                // Verificar salud de Diesel
                let start = Instant::now();
                let diesel_health = database::check_database_health();
                
                // Verificar salud de SQLx
                let sqlx_health_result = match sqlx_database::check_database_health().await {
                    Ok(health) => health,
                    Err(e) => {
                        error!("Error al verificar salud de bases de datos SQLx: {}", e);
                        HashMap::new()
                    }
                };
                
                // Preparar nuevos datos de salud
                let mut health_entries = Vec::new();
                let now = chrono::Utc::now();
                
                // Procesar resultados Diesel
                for (name, healthy) in diesel_health {
                    let response_time = start.elapsed().as_millis() as u64;
                    health_entries.push(DatabaseHealth {
                        name: format!("diesel_{}", name),
                        healthy,
                        last_check: now,
                        response_time_ms: response_time,
                    });
                    
                    if !healthy {
                        warn!("Base de datos Diesel {} no está saludable", name);
                    }
                }
                
                // Procesar resultados SQLx
                for (name, healthy) in sqlx_health_result {
                    let response_time = start.elapsed().as_millis() as u64;
                    health_entries.push(DatabaseHealth {
                        name: format!("sqlx_{}", name),
                        healthy,
                        last_check: now,
                        response_time_ms: response_time,
                    });
                    
                    if !healthy {
                        warn!("Base de datos SQLx {} no está saludable", name);
                    }
                }
                
                // Actualizar estado compartido atómicamente
                {
                    let mut state = shared.lock().await;
                    state.health_data = health_entries;
                    state.last_check = Some(start);
                }
                
                info!("Verificación de salud de bases de datos completada");
            }
        });
    }
    
    // Obtener datos de salud actuales
    pub async fn get_health_data(&self) -> Vec<DatabaseHealth> {
        let state = self.shared.lock().await;
        state.health_data.clone()
    }
    
    // Verificar si todas las bases de datos están saludables
    pub async fn all_healthy(&self) -> bool {
        let state = self.shared.lock().await;
        state.health_data.iter().all(|h| h.healthy)
    }
    
    // Obtener tiempo desde la última verificación
    pub async fn time_since_last_check(&self) -> Option<Duration> {
        let state = self.shared.lock().await;
        state.last_check.map(|t| t.elapsed())
    }
}