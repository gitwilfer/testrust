// Este módulo contiene el controlador para el recurso de usuario.
// Define los handlers para las rutas de usuario.
use actix_web::{web, HttpResponse, Responder, post, get, put, delete, Error};
use crate::application::use_cases::user::{CreateUserUseCase, FindUserByIdUseCase, FindUserByUsernameUseCase, FindAllUsersUseCase, UpdateUserUseCase, DeleteUserUseCase};
use crate::application::dtos::create_user_dto::CreateUserDto;
use crate::application::dtos::update_user_dto::UpdateUserDto;
use std::sync::Arc;
use uuid::Uuid;
use crate::presentation::api::middleware::map_error;
use crate::presentation::api::validators::validate_json;
use crate::presentation::api::responses::{ApiResponse, ApiError};

// Controlador para crear usuarios
pub struct UserController {
    pub create_user_use_case: Arc<CreateUserUseCase<crate::infrastructure::repositories::UserRepositoryImpl>>,
    pub find_user_by_id_use_case: Arc<FindUserByIdUseCase<crate::infrastructure::repositories::UserRepositoryImpl>>,
    pub find_user_by_username_use_case: Arc<FindUserByUsernameUseCase<crate::infrastructure::repositories::UserRepositoryImpl>>,
    pub find_all_users_use_case: Arc<FindAllUsersUseCase<crate::infrastructure::repositories::UserRepositoryImpl>>,
    pub update_user_use_case: Arc<UpdateUserUseCase<crate::infrastructure::repositories::UserRepositoryImpl>>,
    pub delete_user_use_case: Arc<DeleteUserUseCase<crate::infrastructure::repositories::UserRepositoryImpl>>,
}

impl UserController {
    pub fn new(
        create_user_use_case: Arc<CreateUserUseCase<crate::infrastructure::repositories::UserRepositoryImpl>>,
        find_user_by_id_use_case: Arc<FindUserByIdUseCase<crate::infrastructure::repositories::UserRepositoryImpl>>,
        find_user_by_username_use_case: Arc<FindUserByUsernameUseCase<crate::infrastructure::repositories::UserRepositoryImpl>>,
        find_all_users_use_case: Arc<FindAllUsersUseCase<crate::infrastructure::repositories::UserRepositoryImpl>>,
        update_user_use_case: Arc<UpdateUserUseCase<crate::infrastructure::repositories::UserRepositoryImpl>>,
        delete_user_use_case: Arc<DeleteUserUseCase<crate::infrastructure::repositories::UserRepositoryImpl>>,
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

    let user_dto = CreateUserDto {
        username: user_req.username.clone(),
        first_name: user_req.first_name.clone(),
        last_name: user_req.last_name.clone(),
        email: user_req.email.clone(),
        password: user_req.password.clone(),
    };
    let result = user_controller
        .create_user_use_case
        .execute(user_dto.into_inner())
        .await;

    match result {
        Ok(user_response) => Ok(HttpResponse::Created().json(ApiResponse::success(Some(user_response), None))),
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
        Ok(Some(user_response)) => Ok(HttpResponse::Ok().json(ApiResponse::success(Some(user_response), None))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(ApiError::not_found("Usuario no encontrado")))),
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
        Ok(Some(user_response)) => Ok(HttpResponse::Ok().json(ApiResponse::success(Some(user_response), None))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(ApiError::not_found("Usuario no encontrado")))),
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
        Ok(user_responses) => Ok(HttpResponse::Ok().json(ApiResponse::success(Some(user_responses), None))),
        Err(e) => Err(map_error(e)),
    }
}

// Handler para la ruta PUT /api/users/{id}
#[put("/{id}")]
async fn update_user(
    user_controller: web::Data<UserController>,
    id: web::Path<Uuid>,
    user_dto: web::Json<UpdateUserDto>,
) -> Result<HttpResponse, Error> {
    validate_json(&user_dto)?;
    let result = user_controller
        .update_user_use_case
        .execute(id.into_inner(), user_dto.into_inner())
        .await;

    match result {
        Ok(Some(user_response)) => Ok(HttpResponse::Ok().json(ApiResponse::success(Some(user_response), None))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(ApiError::not_found("Usuario no encontrado")))),
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
        Ok(true) => Ok(HttpResponse::NoContent().json(ApiResponse::<()>::success(None, None))),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(ApiError::not_found("Usuario no encontrado")))),
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

/*pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("", web::get().to(find_all_users))
            .route("/{id}", web::get().to(find_user_by_id))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user))
            .route("/username/{username}", web::get().to(find_user_by_username))
    );
}*/
