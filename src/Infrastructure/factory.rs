// src/infrastructure/factory.rs
use std::sync::Arc;
use crate::application::ports::repositories::{UserQueryRepository, UserCommandRepository};
use crate::application::ports::unit_of_work::UnitOfWork;
use crate::application::mappers::UserMapper;
use crate::application::use_cases::user::*;
use crate::infrastructure::repositories::{UserQueryRepositoryImpl, UserCommandRepositoryImpl};
use crate::infrastructure::persistence::DatabaseUnitOfWork;
use crate::infrastructure::auth::AuthServiceImpl;

pub fn create_dependencies() -> AppDependencies {
    // Crear instancias de repositorios
    let user_query_repository = Arc::new(UserQueryRepositoryImpl::new().unwrap());
    let auth_service = Arc::new(AuthServiceImpl::new().unwrap());
    let user_mapper = Arc::new(UserMapper::new());
    
    // Crear UnitOfWork
    let unit_of_work = Arc::new(DatabaseUnitOfWork::new(
        Arc::new(crate::infrastructure::persistence::database::get_pool_from_connection())
    ));
    
    // Crear casos de uso
    let create_user_use_case = Arc::new(CreateUserWithPreferencesUseCase::new(
        unit_of_work.clone(),
        user_query_repository.clone(),
        auth_service.clone(),
        user_mapper.clone(),
    ));
    
    // Otros casos de uso usando los nuevos patrones...
    
    AppDependencies {
        unit_of_work,
        user_query_repository,
        create_user_use_case,
        // Otros servicios y casos de uso...
    }
}

pub struct AppDependencies {
    pub unit_of_work: Arc<dyn UnitOfWork>,
    pub user_query_repository: Arc<dyn UserQueryRepository>,
    pub create_user_use_case: Arc<CreateUserWithPreferencesUseCase>,
    // Otros servicios y casos de uso...
}