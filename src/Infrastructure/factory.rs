use std::sync::Arc;
use anyhow::Result;
use sqlx::Pool;

// Importar UserQueryRepositorySqlx correctamente
use crate::Application::ports::repositories::{UserRepositoryPort, UserQueryRepository, AuthServicePort};
use crate::Application::mappers::UserMapper;
use crate::Application::use_cases::user::*;
use crate::Infrastructure::repositories::{UserRepositoryImpl, UserQueryRepositoryImpl, UserQueryRepositorySqlx};
use crate::Infrastructure::auth::AuthServiceImpl;
use crate::Infrastructure::Persistence::sqlx_database;

// Importamos el caso de uso corregido
use crate::Application::use_cases::user::create_with_preferences::CreateUserWithPreferencesUseCase;

// Función para crear dependencias utilizando SQLx para consultas
pub async fn create_dependencies_with_sqlx() -> Result<AppDependencies> {
    // Obtener pool de SQLx
    let sqlx_pool = sqlx_database::get_default_pool().await?;
    let sqlx_pool_arc = Arc::new(sqlx_pool);
    
    // Crear instancias de repositorios
    let user_repository = Arc::new(UserRepositoryImpl::new()?);
    
    // ⭐ Usar la implementación SQLx para consultas
    let user_query_repository = Arc::new(UserQueryRepositorySqlx::with_pool(sqlx_pool_arc.clone()));
    
    let auth_service = Arc::new(AuthServiceImpl::new()?);
    let user_mapper = Arc::new(UserMapper::new());
    
    // Crear casos de uso utilizando los repositorios
    let create_user_with_preferences_use_case = Arc::new(CreateUserWithPreferencesUseCase::new(
        user_repository.clone(),
        user_query_repository.clone(),
        auth_service.clone(),
        user_mapper.clone(),
    ));

    let login_use_case = Arc::new(LoginUseCase::new(
        user_query_repository.clone(), // Ahora usamos UserQueryRepositorySqlx
        auth_service.clone()
    ));

    // Crear otros casos de uso
    let create_user_use_case = Arc::new(create::CreateUserUseCase::new(
        user_repository.clone(),
        auth_service.clone(),
        user_mapper.clone(),
    ));
    
    let find_user_by_id_use_case = Arc::new(find_by_id::FindUserByIdUseCase::new(
        user_repository.clone(),
        user_mapper.clone(),
    ));
    
    let find_all_users_use_case = Arc::new(find_all::FindAllUsersUseCase::new(
        user_repository.clone(),
        user_mapper.clone(),
    ));
    
    let update_user_use_case = Arc::new(update::UpdateUserUseCase::new(
        user_repository.clone(),
        user_query_repository.clone(),
        user_mapper.clone(),
        auth_service.clone(),
    ));
    
    let delete_user_use_case = Arc::new(delete::DeleteUserUseCase::new(
        user_repository.clone(),
    ));
    
    let find_user_by_username_use_case = Arc::new(find_by_username::FindUserByUsernameUseCase::new(
        user_repository.clone(),
        user_mapper.clone(),
    ));
    
    // Crear y devolver las dependencias
    Ok(AppDependencies {
        user_repository,
        user_query_repository,
        auth_service,
        user_mapper,
        create_user_with_preferences_use_case,
        create_user_use_case,
        find_user_by_id_use_case,
        find_all_users_use_case,
        update_user_use_case,
        delete_user_use_case,
        find_user_by_username_use_case,
        login_use_case,
    })
}

pub fn create_dependencies() -> Result<AppDependencies> {
    // Crear instancias de repositorios
    let user_repository = Arc::new(UserRepositoryImpl::new()?);
    let user_query_repository = Arc::new(UserQueryRepositoryImpl::new()?);
    let auth_service = Arc::new(AuthServiceImpl::new()?);
    let user_mapper = Arc::new(UserMapper::new());
    
    // Crear casos de uso utilizando directamente los repositorios
    let create_user_with_preferences_use_case = Arc::new(CreateUserWithPreferencesUseCase::new(
        user_repository.clone(),
        user_query_repository.clone(),
        auth_service.clone(),
        user_mapper.clone(),
    ));
    
    // Crear otros casos de uso
    let create_user_use_case = Arc::new(create::CreateUserUseCase::new(
        user_repository.clone(),
        auth_service.clone(),
        user_mapper.clone(),
    ));
    
    let find_user_by_id_use_case = Arc::new(find_by_id::FindUserByIdUseCase::new(
        user_repository.clone(),
        user_mapper.clone(),
    ));
    
    let find_all_users_use_case = Arc::new(find_all::FindAllUsersUseCase::new(
        user_repository.clone(),
        user_mapper.clone(),
    ));
    
    let update_user_use_case = Arc::new(update::UpdateUserUseCase::new(
        user_repository.clone(),
        user_mapper.clone(),
        auth_service.clone(),
    ));
    
    let delete_user_use_case = Arc::new(delete::DeleteUserUseCase::new(
        user_repository.clone(),
    ));
    
    let find_user_by_username_use_case = Arc::new(find_by_username::FindUserByUsernameUseCase::new(
        user_repository.clone(),
        user_mapper.clone(),
    ));

    let login_use_case = Arc::new(LoginUseCase::new(
        user_query_repository.clone(), // Usamos UserQueryRepositoryImpl
        auth_service.clone()
    ));
    
    // Crear y devolver las dependencias
    Ok(AppDependencies {
        user_repository,
        user_query_repository,
        auth_service,
        user_mapper,
        create_user_with_preferences_use_case,
        create_user_use_case,
        find_user_by_id_use_case,
        find_all_users_use_case,
        update_user_use_case,
        delete_user_use_case,
        find_user_by_username_use_case,
        login_use_case,
    })
}

pub struct AppDependencies {
    // Repositorios y servicios
    pub user_repository: Arc<dyn UserRepositoryPort>,
    pub user_query_repository: Arc<dyn UserQueryRepository>,
    pub auth_service: Arc<dyn AuthServicePort>,
    pub user_mapper: Arc<UserMapper>,
    
    // Casos de uso
    pub create_user_with_preferences_use_case: Arc<CreateUserWithPreferencesUseCase>,
    pub create_user_use_case: Arc<create::CreateUserUseCase>,
    pub find_user_by_id_use_case: Arc<find_by_id::FindUserByIdUseCase>,
    pub find_all_users_use_case: Arc<find_all::FindAllUsersUseCase>,
    pub update_user_use_case: Arc<update::UpdateUserUseCase>,
    pub delete_user_use_case: Arc<delete::DeleteUserUseCase>,
    pub find_user_by_username_use_case: Arc<find_by_username::FindUserByUsernameUseCase>,
    pub login_use_case: Arc<login::LoginUseCase>,
}