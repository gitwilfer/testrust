// Este módulo contiene el controlador para el recurso de usuario.
// Define los handlers para las rutas de usuario.
use actix_web::{web, HttpResponse, post, get, put, delete, Error};
use crate::application::use_cases::user::{
    CreateUserUseCase, 
    FindUserByIdUseCase, 
    FindUserByUsernameUseCase, 
    FindAllUsersUseCase, 
    UpdateUserUseCase, 
    DeleteUserUseCase
};
use crate::application::dtos::create_user_dto::CreateUserDto;
use crate::application::dtos::update_user_dto::UpdateUserDto;
use std::sync::Arc;
use uuid::Uuid;
use crate::presentation::api::middleware::map_error;
use crate::presentation::api::validators::validate_json;
use crate::presentation::api::responses::{ApiResponse, ApiError};
use crate::presentation::api::models::request::{CreateUserRequest, UpdateUserRequest};
use crate::presentation::api::models::response::UserResponse;

// Controlador para crear usuarios
pub struct UserController {
    // Utilizamos tipos de casos de uso sin especificar implementaciones concretas
    pub create_user_use_case: Arc<dyn CreateUserUseCase>,
    pub find_user_by_id_use_case: Arc<dyn FindUserByIdUseCase>,
    pub find_user_by_username_use_case: Arc<dyn FindUserByUsernameUseCase>,
    pub find_all_users_use_case: Arc<dyn FindAllUsersUseCase>,
    pub update_user_use_case: Arc<dyn UpdateUserUseCase>,
    pub delete_user_use_case: Arc<dyn DeleteUserUseCase>,
}

impl UserController {
    pub fn new(
        create_user_use_case: Arc<dyn CreateUserUseCase>,
        find_user_by_id_use_case: Arc<dyn FindUserByIdUseCase>,
        find_user_by_username_use_case: Arc<dyn FindUserByUsernameUseCase>,
        find_all_users_use_case: Arc<dyn FindAllUsersUseCase>,
        update_user_use_case: Arc<dyn UpdateUserUseCase>,
        delete_user_use_case: Arc<dyn DeleteUserUseCase>,
    ) -> Self {
        UserController {
            create_user_use_case,
            find_user_by_id_use_case,
            find_user_by_username_use_case,
            find_all_users_use_case,
            update_user_use_case,
            delete_user_use_case,
        }
    }
}

// Handler para la ruta POST /api/users
#[post("")]
async fn create_user(
    user_controller: web::Data<UserController>,
    user_req: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, Error> {
    validate_json(&user_req)?;

    // Mapeo explícito de CreateUserRequest a CreateUserDto
    let user_dto = CreateUserDto {
        username: user_req.username.clone(),
        first_name: user_req.first_name.clone(),
        last_name: user_req.last_name.clone(),
        email: user_req.email.clone(),
        password: user_req.password.clone(),
    };
    
    let result = user_controller
        .create_user_use_case
        .execute(user_dto)
        .await;

    match result {
        Ok(user_dto) => {
            // Mapeo explícito de UserResponseDto a UserResponse
            let user_response = UserResponse {
                id: user_dto.id,
                username: user_dto.username,
                first_name: user_dto.first_name,
                last_name: user_dto.last_name,
                email: user_dto.email,
                created_by: user_dto.created_by,
                created_at: user_dto.created_at,
                modified_by: user_dto.modified_by,
                modified_at: user_dto.modified_at,
                status: user_dto.status,
            };
            
            Ok(HttpResponse::Created().json(ApiResponse::success(Some(user_response), None)))
        },
        Err(e) => Err(map_error(e)),
    }
}

// Handler para la ruta GET /api/users/{id}
#[get("/{id}")]
async fn find_user_by_id(
    user_controller: web::Data<UserController>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let result = user_controller
        .find_user_by_id_use_case
        .execute(id.into_inner())
        .await;

    match result {
        Ok(user_dto) => {
            // Mapeo explícito de UserResponseDto a UserResponse
            let user_response = UserResponse {
                id: user_dto.id,
                username: user_dto.username,
                first_name: user_dto.first_name,
                last_name: user_dto.last_name,
                email: user_dto.email,
                created_by: user_dto.created_by,
                created_at: user_dto.created_at,
                modified_by: user_dto.modified_by,
                modified_at: user_dto.modified_at,
                status: user_dto.status,
            };
            
            Ok(HttpResponse::Ok().json(ApiResponse::success(Some(user_response), None)))
        },
        Err(e) => Err(map_error(e)),
    }
}

// Handler para la ruta GET /api/users/username/{username}
#[get("/username/{username}")]
async fn find_user_by_username(
    user_controller: web::Data<UserController>,
    username: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let result = user_controller
        .find_user_by_username_use_case
        .execute(&username.into_inner())
        .await;

    match result {
        Ok(user_dto) => {
            // Mapeo explícito de UserResponseDto a UserResponse
            let user_response = UserResponse {
                id: user_dto.id,
                username: user_dto.username,
                first_name: user_dto.first_name,
                last_name: user_dto.last_name,
                email: user_dto.email,
                created_by: user_dto.created_by,
                created_at: user_dto.created_at,
                modified_by: user_dto.modified_by,
                modified_at: user_dto.modified_at,
                status: user_dto.status,
            };
            
            Ok(HttpResponse::Ok().json(ApiResponse::success(Some(user_response), None)))
        },
        Err(e) => Err(map_error(e)),
    }
}

// Handler para la ruta GET /api/users
#[get("")]
async fn find_all_users(
    user_controller: web::Data<UserController>,
) -> Result<HttpResponse, Error> {
    let result = user_controller
        .find_all_users_use_case
        .execute()
        .await;

    match result {
        Ok(user_dtos) => {
            // Mapeo explícito de cada UserResponseDto a UserResponse
            let user_responses: Vec<UserResponse> = user_dtos
                .into_iter()
                .map(|user_dto| UserResponse {
                    id: user_dto.id,
                    username: user_dto.username,
                    first_name: user_dto.first_name,
                    last_name: user_dto.last_name,
                    email: user_dto.email,
                    created_by: user_dto.created_by,
                    created_at: user_dto.created_at,
                    modified_by: user_dto.modified_by,
                    modified_at: user_dto.modified_at,
                    status: user_dto.status,
                })
                .collect();
            
            Ok(HttpResponse::Ok().json(ApiResponse::success(Some(user_responses), None)))
        },
        Err(e) => Err(map_error(e)),
    }
}

// Handler para la ruta PUT /api/users/{id}
#[put("/{id}")]
async fn update_user(
    user_controller: web::Data<UserController>,
    id: web::Path<Uuid>,
    user_req: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, Error> {
    validate_json(&user_req)?;
    
    // Mapeo explícito de UpdateUserRequest a UpdateUserDto
    let update_dto = UpdateUserDto {
        first_name: user_req.first_name.clone(),
        last_name: user_req.last_name.clone(),
        email: user_req.email.clone(),
        password: user_req.password.clone(),
    };
    
    let result = user_controller
        .update_user_use_case
        .execute(id.into_inner(), update_dto, None)
        .await;

    match result {
        Ok(user_dto) => {
            // Mapeo explícito de UserResponseDto a UserResponse
            let user_response = UserResponse {
                id: user_dto.id,
                username: user_dto.username,
                first_name: user_dto.first_name,
                last_name: user_dto.last_name,
                email: user_dto.email,
                created_by: user_dto.created_by,
                created_at: user_dto.created_at,
                modified_by: user_dto.modified_by,
                modified_at: user_dto.modified_at,
                status: user_dto.status,
            };
            
            Ok(HttpResponse::Ok().json(ApiResponse::success(Some(user_response), None)))
        },
        Err(e) => Err(map_error(e)),
    }
}

// Handler para la ruta DELETE /api/users/{id}
#[delete("/{id}")]
async fn delete_user(
    user_controller: web::Data<UserController>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let result = user_controller
        .delete_user_use_case
        .execute(id.into_inner())
        .await;

    match result {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => Err(map_error(e)),
    }
}

// Configuración de las rutas
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(create_user)
            .service(find_all_users)
            .service(find_user_by_id)
            .service(update_user)
            .service(delete_user)
            .service(find_user_by_username)
    );
}