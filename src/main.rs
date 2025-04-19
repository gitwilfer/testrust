use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use log::{info, LevelFilter, error}; // Añadido error
use env_logger::Builder;
use std::io::Write;
// Usar el nombre del crate 'anyb' para acceder a la biblioteca (lib.rs)
use anyb; // Importar el crate

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // --- 1. Cargar Configuración ---
    // Ventaja: Necesaria antes que nada para configurar logger, pools, etc.
    dotenv().ok();
    let config = anyb::Infrastructure::config::app_config::get_config(); // Usar PascalCase como en lib.rs

    // --- 2. Inicializar Logger ---
    // Ventaja: Permite ver logs desde el inicio, incluyendo la inicialización de pools y contenedor.
    // Usa config.log_level.
    let log_level = match config.log_level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };
    Builder::new()
        .format(|buf, record| { /* ... formato ... */ })
        .filter(None, log_level)
        .init();

    info!("Iniciando aplicación en entorno: {:?}", config.environment);
    info!("Configuración cargada: {:?}", config);

    // --- 3. Inicializar Pools de Conexión (CRÍTICO ANTES DE BUILD) ---
    // Ventaja: Asegura que los pools (Diesel Async y SQLx) estén listos y disponibles
    // ANTES de que el contenedor intente obtenerlos y registrarlos.
    // database_module (llamado por build_app_state) necesita estos pools.
    match anyb::Infrastructure::Persistence::connection_pools::initialize_pools(&config).await { // Añadido .await
        Ok(_) => info!("Pools de bases de datos inicializados correctamente"),
        Err(e) => {
            error!("Error FATAL al inicializar pools de bases de datos: {}", e);
            // Salir si los pools no se pueden inicializar, la app no puede funcionar.
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error de inicialización de BD"));
        }
    }

    // --- 4. Inicializar Mapeos (Opcional, si se usa database_selector) ---
    // Ventaja: Configura mapeos específicos si son necesarios antes de construir dependencias.
    // anyb::Application::services::initialize_database_mappings(); // Usar PascalCase

    // --- 5. Construir el Estado de la Aplicación (Contenedor) ---
    // Ventaja: Ahora que los pools están listos, podemos construir todas las dependencias
    // (UoW, repositorios, casos de uso, controladores) de forma segura y unificada.
    info!("Construyendo AppState...");
    let app_state = match anyb::Container::build_app_state().await { // Llamar a la función unificada
        Ok(state) => {
            info!("AppState construido exitosamente.");
            state // Asigna el AppState construido
        },
        Err(e) => {
            // Si la construcción unificada falla, es un error fatal.
            error!("Error FATAL al construir el AppState: {:?}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error al construir AppState"));
        }
    };

    // --- 6. Preparar Datos para el Servidor ---
    // Ventaja: Clonamos lo necesario ANTES de mover al closure del servidor.
    let server_config = config.clone(); // Clonar config si se usa en el closure
    let app_state_for_server = app_state.clone(); // Clonar AppState para mover

    // --- 7. Iniciar Servidor HTTP ---
    // Ventaja: El servidor solo se inicia si todos los pasos anteriores fueron exitosos.
    info!("Iniciando servidor HTTP en {}:{}", server_config.http_host, server_config.http_port);
    HttpServer::new(move || {
            // El closure 'move' toma posesión de app_state_for_server
            let app_state_clone = app_state_for_server.clone(); // Clonar el Arc para cada worker

            App::new()
                .wrap(Logger::default())
                .app_data(web::Data::new(app_state_clone)) // Pasar el AppState completo
                .configure(anyb::Presentation::api::routes::config) // Configurar rutas
                // ... (swagger opcional) ...
            })
    .bind((server_config.http_host.clone(), server_config.http_port))?
    .workers(4) // Ajustar según necesidad
    .run()
    .await
}
