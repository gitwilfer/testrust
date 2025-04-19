// src/Infrastructure/config/app_config.rs
use std::env;
use std::sync::Arc;
use lazy_static::lazy_static;
use log::info;

use crate::Infrastructure::config::Environment;
use crate::Infrastructure::Persistence::connection_pools::DatabaseConfig;

#[derive(Debug, Clone)]
pub struct AppConfig {
    // Entorno actual
    pub environment: Environment,
    
    // Configuración HTTP
    pub http_host: String,
    pub http_port: u16,
    pub api_base_path: String,
    
    // Configuración JWT
    pub jwt_secret: String,
    pub jwt_expiration: u64,
    
    // Configuración de bases de datos
    pub main_db_config: DatabaseConfig,
    pub analytics_db_config: Option<DatabaseConfig>,
    
    // Configuración de logging
    pub log_level: String,
    
    // Configuración de características
    pub enable_swagger: bool,
    pub enable_metrics: bool,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let environment = Environment::from_env();
        
        info!("Cargando configuración para entorno: {:?}", environment);
        
        // Configuración HTTP
        let http_host = env::var("HTTP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let http_port = env::var("HTTP_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap_or(8080);
        let api_base_path = env::var("API_BASE_PATH").unwrap_or_else(|_| "/api".to_string());
        
        // Configuración JWT
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            if environment.is_prod() {
                panic!("JWT_SECRET must be set in production environment");
            }
            "development_secret_key".to_string()
        });
        
        let jwt_expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "86400".to_string()) // 24 horas por defecto
            .parse::<u64>()
            .unwrap_or(86400);
        
        // Configuración BD principal
        let main_db_url = env::var("MAIN_DATABASE_URL").unwrap_or_else(|_| {
            if environment.is_prod() {
                panic!("MAIN_DATABASE_URL must be set in production environment");
            }
            "postgres://postgres:postgres@localhost/anyb_dev".to_string()
        });
        
        let main_db_config = DatabaseConfig::new(
            "main",
            &main_db_url,
            env::var("MAIN_DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse::<u32>()
                .unwrap_or(10),
        );
        
        // Configuración BD analítica (opcional)
        let analytics_db_config = env::var("ANALYTICS_DATABASE_URL").ok().map(|url| {
            DatabaseConfig::new(
                "analytics",
                &url,
                env::var("ANALYTICS_DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse::<u32>()
                    .unwrap_or(5),
            )
        });
        
        // Configuración de logging
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| {
            match environment {
                Environment::Development => "debug",
                Environment::Testing => "info",
                Environment::Staging => "info",
                Environment::Production => "warn",
            }.to_string()
        });
        
        // Activación de características
        let enable_swagger = env::var("ENABLE_SWAGGER")
            .unwrap_or_else(|_| (!environment.is_prod()).to_string())
            .parse::<bool>()
            .unwrap_or(!environment.is_prod());
            
        let enable_metrics = env::var("ENABLE_METRICS")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);
        
        Self {
            environment,
            http_host,
            http_port,
            api_base_path,
            jwt_secret,
            jwt_expiration,
            main_db_config,
            analytics_db_config,
            log_level,
            enable_swagger,
            enable_metrics,
        }
    }
    
    // Helper para obtener la URL completa del servidor
    pub fn get_server_url(&self) -> String {
        format!("http://{}:{}", self.http_host, self.http_port)
    }
    
    // Helper para obtener la configuración específica para desarrollo
    pub fn is_development(&self) -> bool {
        self.environment.is_dev()
    }
    
    // Helper para obtener la configuración específica para producción
    pub fn is_production(&self) -> bool {
        self.environment.is_prod()
    }
}

// Singleton para acceso global a la configuración
lazy_static! {
    static ref CONFIG: Arc<AppConfig> = Arc::new(AppConfig::from_env());
}

// Función para obtener la configuración
pub fn get_config() -> Arc<AppConfig> {
    CONFIG.clone()
}