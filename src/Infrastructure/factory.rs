use std::sync::Arc;
use crate::application::ports::repositories::{UserRepositoryPort, UserQueryRepository, AuthServicePort};
use crate::application::mappers::UserMapper;
use crate::application::use_cases::user::*;
use crate::infrastructure::repositories::{UserRepositoryImpl, UserQueryRepositoryImpl};
use crate::infrastructure::auth::AuthServiceImpl;

// Importamos el caso de uso corregido
use crate::application::use_cases::user::create_with_preferences::CreateUserWithPreferencesUseCase;

pub fn create_dependencies() -> AppDependencies {
    // Crear instancias de repositorios
    let user_repository = Arc::new(UserRepositoryImpl::new().unwrap());
    let user_query_repository = Arc::new(UserQueryRepositoryImpl::new().unwrap());
    let auth_service = Arc::new(AuthServiceImpl::new().unwrap());
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
    
    AppDependencies {
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
    }
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
}