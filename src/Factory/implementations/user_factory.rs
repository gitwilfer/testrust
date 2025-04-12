use std::sync::Arc;
use anyhow::Result;
use log::{info, debug, trace};

use crate::factory::dependency_provider::DependencyProvider;
use crate::Application::mappers::UserMapper;
use crate::Application::use_cases::user::{
    CreateUserUseCase, FindUserByIdUseCase, FindUserByUsernameUseCase,
    FindAllUsersUseCase, UpdateUserUseCase, DeleteUserUseCase,
    CreateUserWithPreferencesUseCase, FindUserByUsernameOptimizedUseCase
};
use crate::Application::ports::repositories::{
    UserRepositoryPort, UserQueryRepository, UserCommandRepository, AuthServicePort
};
use crate::Infrastructure::repositories::{
    UserRepositoryImpl, UserQueryRepositoryImpl, 
    UserQueryRepositorySqlx, UserCommandRepositoryImpl
};
use crate::Infrastructure::auth::AuthServiceImpl;

/// Registra todos los componentes relacionados con usuarios
pub fn register(provider: &mut dyn DependencyProvider) -> Result<()> {
    debug!("Registrando componentes de usuario...");
    
    // Registrar mapper
    let user_mapper = Arc::new(UserMapper::new());
    provider.register::<UserMapper>(user_mapper.clone());
    trace!("UserMapper registrado");
    
    // Registrar repositorios
    let user_repository = Arc::new(UserRepositoryImpl::new()?);
    provider.register::<dyn UserRepositoryPort>(user_repository.clone());
    trace!("UserRepositoryPort registrado");
    
    let user_query_repository = Arc::new(UserQueryRepositoryImpl::new()?);
    provider.register::<dyn UserQueryRepository>(user_query_repository.clone());
    trace!("UserQueryRepository registrado");
    
    let user_command_repository = Arc::new(UserCommandRepositoryImpl::new()?);
    provider.register::<dyn UserCommandRepository>(user_command_repository.clone());
    trace!("UserCommandRepository registrado");
    
    // Obtener servicio de autenticación
    // En un entorno real, lo obtendríamos del provider si ya está registrado
    let auth_service = if provider.has::<dyn AuthServicePort>() {
        provider.get::<dyn AuthServicePort>().unwrap()
    } else {
        let service = Arc::new(AuthServiceImpl::new()?);
        provider.register::<dyn AuthServicePort>(service.clone());
        service
    };
    
    // Registrar casos de uso
    let create_user_use_case = Arc::new(
        CreateUserUseCase::new(
            user_repository.clone(),
            auth_service.clone(),
            user_mapper.clone()
        )
    );
    provider.register::<CreateUserUseCase>(create_user_use_case);
    trace!("CreateUserUseCase registrado");
    
    let find_user_by_id_use_case = Arc::new(
        FindUserByIdUseCase::new(
            user_repository.clone(),
            user_mapper.clone()
        )
    );
    provider.register::<FindUserByIdUseCase>(find_user_by_id_use_case);
    trace!("FindUserByIdUseCase registrado");
    
    let find_user_by_username_use_case = Arc::new(
        FindUserByUsernameUseCase::new(
            user_repository.clone(),
            user_mapper.clone()
        )
    );
    provider.register::<FindUserByUsernameUseCase>(find_user_by_username_use_case);
    trace!("FindUserByUsernameUseCase registrado");
    
    let find_all_users_use_case = Arc::new(
        FindAllUsersUseCase::new(
            user_repository.clone(),
            user_mapper.clone()
        )
    );
    provider.register::<FindAllUsersUseCase>(find_all_users_use_case);
    trace!("FindAllUsersUseCase registrado");
    
    let update_user_use_case = Arc::new(
        UpdateUserUseCase::new(
            user_repository.clone(),
            user_query_repository.clone(),
            user_mapper.clone(),
            auth_service.clone()
        )
    );
    provider.register::<UpdateUserUseCase>(update_user_use_case);
    trace!("UpdateUserUseCase registrado");
    
    let delete_user_use_case = Arc::new(
        DeleteUserUseCase::new(
            user_repository.clone()
        )
    );
    provider.register::<DeleteUserUseCase>(delete_user_use_case);
    trace!("DeleteUserUseCase registrado");
    
    let create_user_with_preferences_use_case = Arc::new(
        CreateUserWithPreferencesUseCase::new(
            user_repository.clone(),
            user_query_repository.clone(),
            auth_service.clone(),
            user_mapper.clone()
        )
    );
    provider.register::<CreateUserWithPreferencesUseCase>(create_user_with_preferences_use_case);
    trace!("CreateUserWithPreferencesUseCase registrado");
    
    info!("Componentes de usuario registrados correctamente");
    Ok(())
}

/// Registra todos los componentes relacionados con usuarios con soporte SQLx
pub fn register_with_sqlx(provider: &mut dyn DependencyProvider) -> Result<()> {
    debug!("Registrando componentes de usuario con soporte SQLx...");
    
    // Verificar si se ha configurado un pool de SQLx
    let sqlx_pool = match provider.get::<sqlx::Pool<sqlx::Postgres>>() {
        Some(pool) => pool,
        None => {
            debug!("No se encontró pool SQLx en el proveedor, registrando componentes estándar");
            return register(provider);
        }
    };
    
    // Registrar mapper
    let user_mapper = Arc::new(UserMapper::new());
    provider.register::<UserMapper>(user_mapper.clone());
    
    // Registrar repositorios
    let user_repository = Arc::new(UserRepositoryImpl::new()?);
    provider.register::<dyn UserRepositoryPort>(user_repository.clone());
    
    // Usar SQLx para consultas (optimizado)
    let user_query_repository = Arc::new(
        UserQueryRepositorySqlx::with_pool(sqlx_pool.clone())
    );
    provider.register::<dyn UserQueryRepository>(user_query_repository.clone());
    
    let user_command_repository = Arc::new(UserCommandRepositoryImpl::new()?);
    provider.register::<dyn UserCommandRepository>(user_command_repository.clone());
    trace!("Repositorios registrados (SQLx para consultas)");
    
    // Obtener servicio de autenticación
    let auth_service = if provider.has::<dyn AuthServicePort>() {
        provider.get::<dyn AuthServicePort>().unwrap()
    } else {
        let service = Arc::new(AuthServiceImpl::new()?);
        provider.register::<dyn AuthServicePort>(service.clone());
        service
    };
    
    // Registrar caso de uso optimizado para búsqueda por username
    let find_user_by_username_optimized_use_case = Arc::new(
        FindUserByUsernameOptimizedUseCase::new(
            user_query_repository.clone(),
            user_mapper.clone()
        )
    );
    provider.register::<FindUserByUsernameOptimizedUseCase>(find_user_by_username_optimized_use_case);
    trace!("FindUserByUsernameOptimizedUseCase registrado");
    
    // Registrar otros casos de uso
    let create_user_use_case = Arc::new(
        CreateUserUseCase::new(
            user_repository.clone(),
            auth_service.clone(),
            user_mapper.clone()
        )
    );
    provider.register::<CreateUserUseCase>(create_user_use_case);
    
    let find_user_by_id_use_case = Arc::new(
        FindUserByIdUseCase::new(
            user_repository.clone(),
            user_mapper.clone()
        )
    );
    provider.register::<FindUserByIdUseCase>(find_user_by_id_use_case);
    
    let find_all_users_use_case = Arc::new(
        FindAllUsersUseCase::new(
            user_repository.clone(),
            user_mapper.clone()
        )
    );
    provider.register::<FindAllUsersUseCase>(find_all_users_use_case);
    
    let update_user_use_case = Arc::new(
        UpdateUserUseCase::new(
            user_repository.clone(),
            user_query_repository.clone(),
            user_mapper.clone(),
            auth_service.clone()
        )
    );
    provider.register::<UpdateUserUseCase>(update_user_use_case);
    
    let delete_user_use_case = Arc::new(
        DeleteUserUseCase::new(
            user_repository.clone()
        )
    );
    provider.register::<DeleteUserUseCase>(delete_user_use_case);
    
    let create_user_with_preferences_use_case = Arc::new(
        CreateUserWithPreferencesUseCase::new(
            user_repository.clone(),
            user_query_repository.clone(),
            auth_service.clone(),
            user_mapper.clone()
        )
    );
    provider.register::<CreateUserWithPreferencesUseCase>(create_user_with_preferences_use_case);
    trace!("Casos de uso estándar registrados");
    
    info!("Componentes de usuario con soporte SQLx registrados correctamente");
    Ok(())
}