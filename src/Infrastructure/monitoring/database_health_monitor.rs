// src/Infrastructure/monitoring/database_health_monitor.rs

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::interval;
use anyhow::Result;
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

pub struct DatabaseHealthMonitor {
    health_data: Arc<RwLock<Vec<DatabaseHealth>>>,
    check_interval: Duration,
    last_check: Arc<RwLock<Option<Instant>>>,  // Cambiado a Arc<RwLock>
}

impl DatabaseHealthMonitor {
    pub fn new(check_interval_seconds: u64) -> Self {
        Self {
            health_data: Arc::new(RwLock::new(Vec::new())),
            check_interval: Duration::from_secs(check_interval_seconds),
            last_check: Arc::new(RwLock::new(None)),  // Inicializado como Arc<RwLock>
        }
    }
    
    // Iniciar monitoreo en segundo plano
    pub fn start_monitoring(&self) {
        // Creamos clones de los Arc para mover a la tarea asíncrona
        let health_data = self.health_data.clone();
        let check_interval = self.check_interval;
        let last_check = self.last_check.clone();  // Ahora podemos clonar el Arc
        
        tokio::spawn(async move {
            let mut interval = interval(check_interval);
            
            loop {
                interval.tick().await;
                
                // Verificar salud de Diesel
                let start = Instant::now();
                let diesel_health = database::check_database_health();
                
                // Verificar salud de SQLx
                let sqlx_health_result = match sqlx_database::check_database_health().await {
                    Ok(health) => health,
                    Err(e) => {
                        error!("Error al verificar salud de bases de datos SQLx: {}", e);
                        HashMap::new() // Mapa vacío en caso de error
                    }
                };
                
                // Actualizar datos de salud
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
                
                // Actualizar datos compartidos - todo dentro de un bloque async
                {
                    let mut data = health_data.write().await;
                    *data = health_entries;
                }
                
                {
                    let mut last = last_check.write().await;
                    *last = Some(start);
                }
                
                info!("Verificación de salud de bases de datos completada");
            }
        });
    }
    
    // Obtener datos de salud actuales
    pub async fn get_health_data(&self) -> Vec<DatabaseHealth> {
        self.health_data.read().await.clone()
    }
    
    // Verificar si todas las bases de datos están saludables
    pub async fn all_healthy(&self) -> bool {
        let data = self.health_data.read().await;
        data.iter().all(|h| h.healthy)
    }
    
    // Obtener tiempo desde la última verificación
    pub async fn time_since_last_check(&self) -> Option<Duration> {
        let last_check = self.last_check.read().await;
        last_check.map(|t| t.elapsed())
    }
}