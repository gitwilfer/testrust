// src/Infrastructure/monitoring/database_health_monitor.rs

use std::sync::{Arc, Mutex}; // ⭐ Usamos std::sync::Mutex en lugar de tokio::sync
use std::time::{Duration, Instant};
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

pub struct DatabaseHealthMonitor {
    // ⭐ Usamos std::sync::Mutex en lugar de tokio::sync para evitar bloqueos asíncronos en el spawn
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
                
                // ⭐ Ejecutamos check_database_health de manera segura
                // Al hacerlo en una función separada, evitamos que la captura sea problemática
                let diesel_health = collect_diesel_health();
                
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
                let start = Instant::now();
                
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
                
                // Actualizar datos compartidos con locks sincronos (no asíncronos)
                if let Ok(mut data) = health_data.lock() {
                    *data = health_entries;
                }
                
                if let Ok(mut last) = last_check.lock() {
                    *last = Some(start);
                }
                
                info!("Verificación de salud de bases de datos completada");
            }
        });
    }
    
    // Obtener datos de salud actuales
    pub async fn get_health_data(&self) -> Vec<DatabaseHealth> {
        match self.health_data.lock() {
            Ok(data) => data.clone(),
            Err(_) => Vec::new(), // En caso de error, retornar vector vacío
        }
    }
    
    // Verificar si todas las bases de datos están saludables
    pub async fn all_healthy(&self) -> bool {
        match self.health_data.lock() {
            Ok(data) => data.iter().all(|h| h.healthy),
            Err(_) => false, // Si hay error, considerar no saludable
        }
    }
    
    // Obtener tiempo desde la última verificación
    pub async fn time_since_last_check(&self) -> Option<Duration> {
        match self.last_check.lock() {
            Ok(last) => last.map(|t| t.elapsed()),
            Err(_) => None,
        }
    }
}

// ⭐ Función separada para ejecutar check_database_health
// Esto evita capturar potenciales referencias problemáticas
fn collect_diesel_health() -> HashMap<String, bool> {
    database::check_database_health()
}