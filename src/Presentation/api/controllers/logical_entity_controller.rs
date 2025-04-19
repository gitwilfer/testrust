use actix_web::{web, HttpResponse, post, Error};
use std::sync::Arc;
use uuid::Uuid;
use log::{info, error};

use crate::Container::app_state::AppState;
use crate::Application::use_cases::logical_entities::{ // Usar los nombres correctos exportados
    CreateEntityWithAttributesUseCase, // Nombre correcto del trait
    CreateEntityWithAttributesCommand, // Nombre correcto del comando
    CreateEntityError, // Asumiendo que este es el nombre correcto del error exportado
};
use crate::Presentation::api::validators::validate_json;
use crate::Presentation::api::responses::ApiResponse;
use crate::Presentation::api::models::request::CreateEntityWithAttributesRequest;
use crate::Presentation::api::models::response::CreateLogicalEntityResponse; // O LogicalEntityResponse si devuelves más datos
use crate::Presentation::api::adapters::ErrorAdapter;
// Probablemente necesites importar el trait CommandHandler si lo usas genéricamente
// use crate::Application::use_cases::common::CommandHandler;

// Controlador para entidades lógicas
pub struct LogicalEntityController {
    // Asume que tienes un trait y una implementación para el caso de uso
    pub create_logical_entity_use_case: Arc<dyn CreateEntityWithAttributesUseCase>, // Usar el trait correcto
    // Añade otros casos de uso (find, update, delete) aquí cuando los necesites
}

impl LogicalEntityController {
    pub fn new(create_logical_entity_use_case: Arc<dyn CreateEntityWithAttributesUseCase>) -> Self { // Usar el trait correcto
        Self {
            create_logical_entity_use_case,
        }
    }
}

#[post("")]
async fn create_logical_entity(
    app_state: web::Data<AppState>,
    req_payload: web::Json<CreateEntityWithAttributesRequest>,
    // --- ¡¡¡IMPORTANTE: OBTENER USER ID!!! ---
    // Aquí necesitarás extraer el ID del usuario autenticado.
    // Esto normalmente se hace a través de datos añadidos por un middleware de autenticación.
    // Ejemplo conceptual (requiere que AuthMiddleware funcione y añada `user_id` a las extensiones):
    // req: HttpRequest,
) -> Result<HttpResponse, Error> {
    // Validar request
    validate_json(&req_payload)?;

    // --- Placeholder para obtener User ID ---
    // ¡¡¡REEMPLAZAR ESTO CON LA EXTRACCIÓN REAL DEL USUARIO AUTENTICADO!!!
    // let user_id = req.extensions().get::<Uuid>().cloned()
    //     .ok_or_else(|| {
    //         error!("User ID not found in request extensions. Is AuthMiddleware running correctly?");
    //         actix_web::error::ErrorInternalServerError("Authentication context missing")
    //     })?;
    // Por ahora, usamos un placeholder como en tu ejemplo original, pero ¡esto es incorrecto!
    let user_id_placeholder = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    // --- Fin Placeholder ---

    info!("Creando nueva entidad lógica: {}", req_payload.name);

    // Mapear Request a Comando de Aplicación
    let command = CreateEntityWithAttributesCommand { // Usar el struct de comando correcto
        entity_name: req_payload.name.clone(), // El comando espera 'entity_name'
        attributes: req_payload.attributes.clone(), // El comando espera 'attributes'
        // description y assign_view no están en CreateEntityWithAttributesCommand
        created_by_user_id: user_id_placeholder, // <--- ¡USA EL user_id REAL AQUÍ!
    };

    // Ejecutar caso de uso
    // Acceder al controlador específico desde AppState (necesitarás añadirlo)
    match app_state.logical_entity_controller_data.create_logical_entity_use_case.execute(command).await {
        Ok(entity_id) => {
            info!("Entidad lógica creada con éxito: ID={}", entity_id);

            // Mapear resultado a Response
            let response_body = CreateLogicalEntityResponse { id: entity_id };

            Ok(HttpResponse::Created().json(ApiResponse::success(Some(response_body), Some("Logical entity created successfully."))))
        },
        Err(app_error) => {
            // Asegúrate que ErrorAdapter maneje los errores de CreateLogicalEntityError
            error!("Error al crear entidad lógica: {:?}", app_error);
            Ok(ErrorAdapter::map_application_error(app_error))
        },
    }
}

// Configuración de las rutas para este controlador
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("") // El prefijo se define en routes.rs
            .service(create_logical_entity)
            // Añade aquí los servicios para find, update, delete cuando los implementes
    );
}
