// src/main.rs
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::{info, LevelFilter};
use env_logger::Builder;
use std::io::Write;
use std::sync::Arc;

// Importación explícita del módulo de rutas
use crate::presentation::api::routes;
use crate::infrastructure::config::app_config;
use crate::infrastructure::persistence::database;
use crate::application::services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Cargar variables de entorno
    dotenv().ok();

    // Obtener configuración
    let config = infrastructure::config::app_config::get_config();
    
    // Inicializar el logger con nivel basado en configuración
    let log_level = match config.log_level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };
    
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, log_level)
        .init();

    info!("Iniciando aplicación en entorno: {:?}", config.environment);
    info!("Configuración cargada: {:?}", config);

    // Inicializar conexiones a bases de datos usando la configuración
    match infrastructure::persistence::database::initialize_with_config(&config) {
        Ok(_) => info!("Conexiones a bases de datos inicializadas correctamente"),
        Err(e) => {
            log::error!("Error al inicializar conexiones a bases de datos: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error de inicialización de BD"));
        }
    }

    // Inicializar mapeado de entidades a bases de datos
    application::services::initialize_database_mappings();

    // Clonar config para usar después de la closure
    let server_config = config.clone();

    info!("Iniciando servidor HTTP en {}:{}", config.http_host, config.http_port);
    
    HttpServer::new(move || {
        let mut app = App::new();
        
        // Configurar rutas API usando la importación correcta
        app = app.configure(routes::config);
        
        // Añadir Swagger si está activado
        if config.enable_swagger {
            info!("Swagger UI habilitado en /swagger-ui");
            // Aquí iría la configuración de Swagger
        }
        
        app
    })
    .bind((server_config.http_host.clone(), server_config.http_port))?
    .run()
    .await
}