use actix_web::{web, HttpResponse, post, get, put, delete, Error};
use crate::Container::app_state::AppState; // Importar AppState
use crate::Application::use_cases::{
    CreateUserUseCase, 
    FindUserByIdUseCase, 
    FindUserByUsernameUseCase, 
    FindAllUsersUseCase, 
    UpdateUserUseCase, 
    DeleteUserUseCase
};
use crate::Application::dtos::create_user_dto::CreateUserDto;
use crate::Application::dtos::update_user_dto::UpdateUserDto;
use std::sync::Arc;
use uuid::Uuid;
use log::{info, error};
use crate::Presentation::api::adapters::ErrorAdapter;
use crate::Presentation::api::validators::validate_json;
use crate::Presentation::api::responses::ApiResponse;
use crate::Presentation::api::models::request::{CreateUserRequest, UpdateUserRequest};
use crate::Presentation::api::models::response::UserResponse;

// Controlador para usuarios
pub struct UserController {
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
    app_state: web::Data<AppState>, // Cambiar a AppState
    user_req: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, Error> {
    // Validar request
    validate_json(&user_req)?;

    info!("Creando nuevo usuario: {}", user_req.username);

    // Mapeo explícito de CreateUserRequest a CreateUserDto
    let user_dto = CreateUserDto {
        username: user_req.username.clone(),
        first_name: user_req.first_name.clone(),
        last_name: user_req.last_name.clone(),
        email: user_req.email.clone(),
        password: user_req.password.clone(),
    };
    
    // Ejecutar caso de uso
    // Acceder al controlador específico desde AppState
    match app_state.user_controller_data.create_user_use_case.execute(user_dto).await {
        Ok(user_dto) => {
            info!("Usuario creado con éxito: ID={}", user_dto.id);
            
            // Mapeo explícito de UserResponseDto a UserResponse
            let user_response = UserResponse {
                id: user_dto.id,
                username: user_dto.username,
                first_name: user_dto.first_name,
                last_name: user_dto.last_name,
                email: user_dto.email,
                created_by: user_dto.created_by,
                created_at: user_dto.created_at,
                updated_by: user_dto.updated_by,
                updated_at: user_dto.updated_at,
                status: user_dto.status,
            };
            
            Ok(HttpResponse::Created().json(ApiResponse::success(Some(user_response), None)))
        },
        Err(app_error) => {
            error!("Error al crear usuario: {:?}", app_error);
            Ok(ErrorAdapter::map_application_error(app_error))
        },
    }
}

// Handler para la ruta GET /api/users/{id}
#[get("/{id}")]
async fn find_user_by_id(
    app_state: web::Data<AppState>, // Cambiar a AppState
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = id.into_inner();
    info!("Buscando usuario por ID: {}", user_id);
    
    // Acceder al controlador específico desde AppState
    match app_state.user_controller_data.find_user_by_id_use_case.execute(user_id).await {
        Ok(user_dto) => {
            info!("Usuario encontrado: ID={}", user_dto.id);
            
            // Mapeo explícito de UserResponseDto a UserResponse
            let user_response = UserResponse {
                id: user_dto.id,
                username: user_dto.username,
                first_name: user_dto.first_name,
                last_name: user_dto.last_name,
                email: user_dto.email,
                created_by: user_dto.created_by,
                created_at: user_dto.created_at,
                updated_by: user_dto.updated_by,
                updated_at: user_dto.updated_at,
                status: user_dto.status,
            };
            
            Ok(HttpResponse::Ok().json(ApiResponse::success(Some(user_response), None)))
        },
        Err(app_error) => {
            error!("Error al buscar usuario por ID {}: {:?}", user_id, app_error);
            Ok(ErrorAdapter::map_application_error(app_error))
        },
    }
}

// Handler para la ruta GET /api/users/username/{username}
#[get("/username/{username}")]
async fn find_user_by_username(
    app_state: web::Data<AppState>, // Cambiar a AppState
    username: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let username_value = username.into_inner();
    info!("Buscando usuario por username: {}", username_value);
    
    // Acceder al controlador específico desde AppState
    match app_state.user_controller_data.find_user_by_username_use_case.execute(&username_value).await {
        Ok(user_dto) => {
            info!("Usuario encontrado por username {}: ID={}", username_value, user_dto.id);
            
            // Mapeo explícito de UserResponseDto a UserResponse
            let user_response = UserResponse {
                id: user_dto.id,
                username: user_dto.username,
                first_name: user_dto.first_name,
                last_name: user_dto.last_name,
                email: user_dto.email,
                created_by: user_dto.created_by,
                created_at: user_dto.created_at,
                updated_by: user_dto.updated_by,
                updated_at: user_dto.updated_at,
                status: user_dto.status,
            };
            
            Ok(HttpResponse::Ok().json(ApiResponse::success(Some(user_response), None)))
        },
        Err(app_error) => {
            error!("Error al buscar usuario por username {}: {:?}", username_value, app_error);
            Ok(ErrorAdapter::map_application_error(app_error))
        },
    }
}

// Handler para la ruta GET /api/users
#[get("")]
async fn find_all_users(
    app_state: web::Data<AppState>, // Cambiar a AppState
) -> Result<HttpResponse, Error> {
    info!("Obteniendo todos los usuarios");
    
    // Acceder al controlador específico desde AppState
    match app_state.user_controller_data.find_all_users_use_case.execute().await {
        Ok(user_dtos) => {
            info!("Se encontraron {} usuarios", user_dtos.len());
            
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
                    updated_by: user_dto.updated_by,
                    updated_at: user_dto.updated_at,
                    status: user_dto.status,
                })
                .collect();
            
            Ok(HttpResponse::Ok().json(ApiResponse::success(Some(user_responses), None)))
        },
        Err(app_error) => {
            error!("Error al obtener todos los usuarios: {:?}", app_error);
            Ok(ErrorAdapter::map_application_error(app_error))
        },
    }
}

// Handler para la ruta PUT /api/users/{id}
#[put("/{id}")]
async fn update_user(
    app_state: web::Data<AppState>, // Cambiar a AppState
    id: web::Path<Uuid>,
    user_req: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, Error> {
    // Validar request
    validate_json(&user_req)?;
    
    let user_id = id.into_inner();
    info!("Actualizando usuario con ID: {}", user_id);
    
    // Mapeo explícito de UpdateUserRequest a UpdateUserDto
    let update_dto = UpdateUserDto {
        first_name: user_req.first_name.clone(),
        last_name: user_req.last_name.clone(),
        email: user_req.email.clone(),
        password: user_req.password.clone(),
    };
    
    // Ejecutar caso de uso
    // Acceder al controlador específico desde AppState
    match app_state.user_controller_data.update_user_use_case.execute(user_id, update_dto, None).await {
        Ok(user_dto) => {
            info!("Usuario actualizado con éxito: ID={}", user_dto.id);
            
            // Mapeo explícito de UserResponseDto a UserResponse
            let user_response = UserResponse {
                id: user_dto.id,
                username: user_dto.username,
                first_name: user_dto.first_name,
                last_name: user_dto.last_name,
                email: user_dto.email,
                created_by: user_dto.created_by,
                created_at: user_dto.created_at,
                updated_by: user_dto.updated_by,
                updated_at: user_dto.updated_at,
                status: user_dto.status,
            };
            
            Ok(HttpResponse::Ok().json(ApiResponse::success(Some(user_response), None)))
        },
        Err(app_error) => {
            error!("Error al actualizar usuario {}: {:?}", user_id, app_error);
            Ok(ErrorAdapter::map_application_error(app_error))
        },
    }
}

// Handler para la ruta DELETE /api/users/{id}
#[delete("/{id}")]
async fn delete_user(
    app_state: web::Data<AppState>, // Cambiar a AppState
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = id.into_inner();
    info!("Eliminando usuario con ID: {}", user_id);
    
    // Acceder al controlador específico desde AppState
    match app_state.user_controller_data.delete_user_use_case.execute(user_id).await {
        Ok(()) => {
            info!("Usuario eliminado con éxito: ID={}", user_id);
            
            // Para mantener el patrón consistente, devolvemos un ApiResponse
            // con success: true pero data: None (porque el delete no devuelve datos)
            Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(None, Some("Usuario eliminado con éxito"))))
        },
        Err(app_error) => {
            error!("Error al eliminar usuario {}: {:?}", user_id, app_error);
            Ok(ErrorAdapter::map_application_error(app_error))
        },
    }
}

// Configuración de las rutas
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(create_user)
            .service(find_all_users)
            .service(find_user_by_id)
            .service(update_user)
            .service(delete_user)
            .service(find_user_by_username)
    );
}