// 'container' es parte de la biblioteca (lib.rs), no se declara aquí.
// Se accede a través de 'anyb::container'

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::{info, LevelFilter};
use env_logger::Builder;
use std::io::Write;
use std::sync::Arc;
// Usar el nombre del crate 'anyb' para acceder a la biblioteca (lib.rs)
// Las rutas a infrastructure, application, presentation se usarán completas

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Cargar variables de entorno
    dotenv().ok();

    // Obtener configuración
    let config = anyb::Infrastructure::config::app_config::get_config(); // Corregir capitalización
    
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
    match anyb::Infrastructure::Persistence::database::initialize_with_config(&config) {
        Ok(_) => info!("Conexiones a bases de datos Diesel inicializadas correctamente"),
        Err(e) => {
            log::error!("Error al inicializar conexiones a bases de datos Diesel: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error de inicialización de BD"));
        }
    }

    // Inicializar conexiones SQLx
    match anyb::Infrastructure::Persistence::sqlx_database::initialize_with_config(&config).await {
        Ok(_) => info!("Conexiones a bases de datos SQLx inicializadas correctamente"),
        Err(e) => {
            log::error!("Error al inicializar conexiones a bases de datos SQLx: {}", e);
            log::warn!("Continuando con Diesel para operaciones de base de datos");
        }
    }
    // Inicializar mapeado de entidades a bases de datos
    anyb::Application::services::initialize_database_mappings(); // Corregir capitalización

    // Clonar el config para usarlo después del move
    let server_config = config.clone();

    info!("Iniciando servidor HTTP en {}:{}", config.http_host, config.http_port);
    
    // --- Construir el estado de la aplicación --- ANTES de HttpServer::new
    // CAMBIO: Usar el nuevo módulo container en lugar del original container.rs
    let app_state = match anyb::container::build_with_sqlx().await {
        Ok(state) => {
            info!("Aplicación inicializada con SQLx para consultas");
            state
        },
        Err(e) => {
            log::warn!("No se pudo inicializar con SQLx: {}. Usando implementación Diesel", e);
            anyb::container::build().await
                .expect("Error fatal al construir el estado de la aplicación")
        }
    };

    // Clonamos el estado ANTES de moverlo a la clausura.
    let app_state_for_server = app_state.clone();
    
    HttpServer::new(move || {
            // Usamos el estado clonado que fue movido a la clausura.
            let app_state_clone = app_state_for_server.clone();

            App::new()
                // Registrar los datos compartidos desde AppState
                .app_data(app_state_clone.auth_controller_data.clone())
                .app_data(app_state_clone.user_controller_data.clone())
                .app_data(app_state_clone.health_controller_data.clone())
                // Aquí registrarías otros datos de app_state_clone si los hubiera

                // Configurar rutas API (usando la ruta completa)
                .configure(anyb::Presentation::api::routes::config) // Corregir capitalización
    
                    // Añadir Swagger si está activado
                    // (Considera mover esta lógica también a una función de configuración si crece)
                    // .configure(|cfg| {
                    //     // Necesitamos clonar server_config o config para usarla aquí si se descomenta
                    //     let config_clone = server_config.clone();
                    //     if config_clone.enable_swagger {
                    //         info!("Swagger UI enabled at /swagger-ui");
                    //     }
                    // })
            })
    .bind((server_config.http_host.clone(), server_config.http_port))?
    .workers(4)
    .run()
    .await
}