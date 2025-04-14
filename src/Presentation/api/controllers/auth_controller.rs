use actix_web::{web, HttpResponse, post, Error};
use std::sync::Arc;
use crate::Container::AppState; // Importar AppState

use crate::Application::dtos::auth_dto::LoginDto;
use crate::Application::use_cases::traits::LoginUseCase;
use crate::Presentation::api::validators::validate_json;
use crate::Presentation::api::responses::ApiResponse;
use crate::Presentation::api::models::request::LoginRequest;
use crate::Presentation::api::models::response::TokenResponse;
use crate::Presentation::api::adapters::ErrorAdapter;

pub struct AuthController {
    pub login_use_case: Arc<dyn LoginUseCase>,
}

impl AuthController {
    pub fn new(login_use_case: Arc<dyn LoginUseCase>) -> Self {
        AuthController {
            login_use_case,
        }
    }
}

#[post("/login")]
async fn login(
    app_state: web::Data<AppState>, // Cambiar a AppState
    login_req: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    // Validar request
    validate_json(&login_req)?;
    
    // Mapear request a DTO
    let login_dto = LoginDto {
        username: login_req.username.clone(),
        password: login_req.password.clone(),
    };
    
    // Ejecutar caso de uso
    // Acceder al controlador específico desde AppState
    let result = app_state.auth_controller_data
        .login_use_case
        .execute(login_dto)
        .await;
    
    match result {
        Ok(token_dto) => {
            // Mapear DTO a response
            let token_response = TokenResponse {
                access_token: token_dto.access_token,
                token_type: token_dto.token_type,
                expires_in: token_dto.expires_in,
                user_id: token_dto.user_id,
            };
            
            Ok(HttpResponse::Ok().json(ApiResponse::success(Some(token_response), None)))
        },
        Err(app_error) => {
            Ok(ErrorAdapter::map_application_error(app_error))
        },
    }
}

// Configuración de las rutas
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login)
    );
}