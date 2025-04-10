 // 'container' es parte de la biblioteca (lib.rs), no se declara aquí.
 // Se accede a través de 'anyb::container'

 use actix_web::{web, App, HttpServer};
 use dotenv::dotenv;
 use log::{info, LevelFilter};
 use env_logger::Builder;
 use std::io::Write;
 use std::sync::Arc;
 // Usar el nombre del crate 'anyb' para acceder a la biblioteca (lib.rs)
 use anyb::container::AppState;
 use anyb::infrastructure;
 use anyb::application;
 use anyb::presentation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Cargar variables de entorno
    dotenv().ok();

    // Obtener configuración
    let config = anyb::infrastructure::config::app_config::get_config();
    
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
    match anyb::infrastructure::persistence::database::initialize_with_config(&config) {
        Ok(_) => info!("Conexiones a bases de datos inicializadas correctamente"),
        Err(e) => {
            log::error!("Error al inicializar conexiones a bases de datos: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error de inicialización de BD"));
        }
    }

    // Inicializar mapeado de entidades a bases de datos
    anyb::application::services::initialize_database_mappings();

    // Clonar el config para usarlo después del move
    let server_config = config.clone();

    info!("Iniciando servidor HTTP en {}:{}", config.http_host, config.http_port);
    
        // --- Construir el estado de la aplicación --- ANTES de HttpServer::new
        // La construcción de dependencias ahora está encapsulada en AppState::build()
        let app_state = match AppState::build() {
            Ok(state) => state,
            Err(e) => {
                log::error!("Error al construir el estado de la aplicación: {:?}", e);
                // Podrías usar std::process::exit(1) o retornar un error específico aquí
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error de inicialización de estado"));
            }
        };
    
        HttpServer::new(move || {
                // Clonamos el AppState (capturado por 'move') para cada worker.
                let app_state_clone = app_state.clone();
    
                App::new()
                    // Registrar los datos compartidos desde AppState
                    .app_data(app_state_clone.auth_controller_data.clone())
                    // Aquí registrarías otros datos de app_state_clone si los hubiera
                    // .app_data(app_state_clone.user_controller_data.clone())
    
                    // Configurar rutas API (usando el módulo presentation importado)
                    .configure(anyb::presentation::api::routes::config) // Usar anyb::presentation
    
                    // Añadir Swagger si está activado
                    // (Considera mover esta lógica también a una función de configuración si crece)
                    // .configure(|cfg| {
                    //     // Necesitamos clonar server_config o config para usarla aquí si se descomenta
                    //     let config_clone = server_config.clone();
                    //     if config_clone.enable_swagger {
                    //         info!("Swagger UI enabled at /swagger-ui");
                    //         // Configuración de Swagger aquí...
                    //     }
                    // })
            })
    .bind((server_config.http_host.clone(), server_config.http_port))?
    .run()
    .await
}