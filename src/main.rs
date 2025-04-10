 // No declarar módulos aquí, ya están en src/lib.rs
 mod container; // Añadir declaración del módulo container

 use actix_web::{web, App, HttpServer}; // Añadido web
 use dotenv::dotenv;
 use log::{info, LevelFilter};
 use env_logger::Builder;
 use std::io::Write;
 use std::sync::Arc;
 use crate::container::AppState; // Importar AppState desde el crate
 use crate::infrastructure; // Importar para acceder a sub-módulos como config y persistence
 use crate::application; // Importar para acceder a sub-módulos como services
 use crate::presentation; // Importar para acceder a sub-módulos como api

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

    // Clonar el config para usarlo después del move
    let server_config = config.clone();

    info!("Iniciando servidor HTTP en {}:{}", config.http_host, config.http_port);
    
    HttpServer::new(move || {
            // Clonamos el AppState para cada worker del servidor HTTP.
            // AppState deriva Clone, por lo que esto es eficiente (clona Arcs/web::Data).
            // Nota: La creación de app_state se movió fuera de la clausura.
            let app_state_clone = app_state.clone();
    
            App::new()
                // Registrar los datos compartidos desde AppState
                .app_data(app_state_clone.auth_controller_data.clone())
                // Aquí registrarías otros datos de app_state_clone si los hubiera
                // .app_data(app_state_clone.user_controller_data.clone())
    
                // Configurar rutas API (usando el módulo presentation importado)
                .configure(presentation::api::routes::config)
    
                // Añadir Swagger si está activado
                // (Considera mover esta lógica también a una función de configuración si crece)
                // .configure(|cfg| {
                //     if server_config.enable_swagger { // Necesitamos clonar server_config o config para usarla aquí
                //         info!("Swagger UI enabled at /swagger-ui");
                //         // Configuración de Swagger aquí...
                //     }
                // })
        })
    .bind((server_config.http_host.clone(), server_config.http_port))?
    .run()
    .await
}