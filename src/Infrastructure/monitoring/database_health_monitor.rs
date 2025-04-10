use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::interval;
use log::{info, warn, error};
use std::collections::HashMap;

use crate::Infrastructure::Persistence::database;
use crate::Infrastructure::Persistence::sqlx_database;

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
                // Clonamos para evitar capturar referencias
                let diesel_health = database::check_database_health().clone();
                
                // Verificar salud de SQLx
                // La función ya ha sido corregida para ser segura
                let sqlx_health = match sqlx_database::check_database_health().await {
                    Ok(health) => health,
                    Err(e) => {
                        error!("Error al verificar salud de bases de datos SQLx: {}", e);
                        HashMap::new()
                    }
                };
                
                // Preparar nuevos datos de salud
                let mut health_entries = Vec::new();
                
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
                for (name, healthy) in sqlx_health {
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