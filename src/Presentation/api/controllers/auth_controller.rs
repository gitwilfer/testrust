use actix_web::{web, HttpResponse, post, Error};
use std::sync::Arc;

use crate::Application::dtos::auth_dto::LoginDto;
use crate::Application::use_cases::traits::LoginUseCase;
use crate::presentation::api::middleware::map_error;
use crate::presentation::api::validators::validate_json;
use crate::presentation::api::responses::ApiResponse;
use crate::presentation::api::models::request::LoginRequest;
use crate::presentation::api::models::response::TokenResponse;
use crate::presentation::api::adapters::ErrorAdapter;

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
    auth_controller: web::Data<AuthController>,
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
    let result = auth_controller
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