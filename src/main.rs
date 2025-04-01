mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::info;
use clap::Parser;
// use presentation::cli::Cli;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Cargar variables de entorno
    dotenv().ok();

    // Inicializar el logger
    env_logger::init();

    // Inicializar conexiones a bases de datos
    match infrastructure::persistence::database::initialize_databases() {
        Ok(_) => info!("Conexiones a bases de datos inicializadas correctamente"),
        Err(e) => {
            log::error!("Error al inicializar conexiones a bases de datos: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error de inicialización de BD"));
        }
    }

    // Parse command-line arguments
    // let cli = Cli::parse();

    // Run the CLI or the API
    // match cli.command {
    //     command => {
    //         presentation::cli::execute_command(command).await;
    //     }
    // }

    HttpServer::new(move || {
        App::new()
            .configure(presentation::api::routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
