use std::sync::Arc;
use anyhow::Result;
use log::{info, debug, warn};

use crate::container::builder::ContainerBuilder;
use crate::Application::mappers::UserMapper;
use crate::Application::use_cases::user::{
    CreateUserUseCase, FindUserByIdUseCase, FindUserByUsernameUseCase,
    FindAllUsersUseCase, UpdateUserUseCase, DeleteUserUseCase,
    FindUserByUsernameOptimizedUseCase
};
use crate::Infrastructure::repositories::{
    UserRepositoryImpl, UserQueryRepositoryImpl, 
    UserQueryRepositorySqlx, UserCommandRepositoryImpl
};
use crate::Infrastructure::auth::AuthServiceImpl;
use crate::Presentation::api::controllers::UserController;

/// Registra todos los componentes relacionados con el dominio de usuarios
/// utilizando la implementación estándar basada en Diesel
pub fn register(builder: &mut ContainerBuilder) -> Result<()> {
    debug!("Registrando componentes del módulo de usuarios");
    
    // Registrar el mapper
    let user_mapper = Arc::new(UserMapper::new());
    builder.register_service(user_mapper.clone());
    debug!("Mapper de usuarios registrado");
    
    // Registrar los repositorios
    let user_repository = Arc::new(
        UserRepositoryImpl::new()?
    );
    builder.register_service(user_repository.clone());
    debug!("Repositorio de usuarios registrado");
    
    let user_query_repository = Arc::new(
        UserQueryRepositoryImpl::new()?
    );
    builder.register_service(user_query_repository.clone());
    debug!("Repositorio de consultas de usuarios registrado");
    
    let user_command_repository = Arc::new(
        UserCommandRepositoryImpl::new()?
    );
    builder.register_service(user_command_repository.clone());
    debug!("Repositorio de comandos de usuarios registrado");
    
    // Obtener el servicio de autenticación (se registrará en auth_module)
    // Por ahora lo creamos aquí para mantener la consistencia
    let auth_service = Arc::new(
        AuthServiceImpl::new()?
    );
    
    // Registrar los casos de uso
    let create_user_use_case = Arc::new(
        CreateUserUseCase::new(
            user_repository.clone(),
            auth_service.clone(),
            user_mapper.clone()
        )
    );
    builder.register_service(create_user_use_case.clone());
    
    let find_user_by_id_use_case = Arc::new(
        FindUserByIdUseCase::new(
            user_repository.clone(),
            user_mapper.clone()
        )
    );
    builder.register_service(find_user_by_id_use_case.clone());
    
    let find_user_by_username_use_case = Arc::new(
        FindUserByUsernameUseCase::new(
            user_repository.clone(),
            user_mapper.clone()
        )
    );
    builder.register_service(find_user_by_username_use_case.clone());
    
    let find_all_users_use_case = Arc::new(
        FindAllUsersUseCase::new(
            user_repository.clone(),
            user_mapper.clone()
        )
    );
    builder.register_service(find_all_users_use_case.clone());
    
    let update_user_use_case = Arc::new(
        UpdateUserUseCase::new(
            user_repository.clone(),
            user_query_repository.clone(),
            user_mapper.clone(),
            auth_service.clone()
        )
    );
    builder.register_service(update_user_use_case.clone());
    
    let delete_user_use_case = Arc::new(
        DeleteUserUseCase::new(
            user_repository.clone()
        )
    );
    builder.register_service(delete_user_use_case.clone());
    debug!("Casos de uso de usuarios registrados");
    
    // Registrar el controlador
    let user_controller = UserController::new(
        create_user_use_case,
        find_user_by_id_use_case,
        find_user_by_username_use_case,
        find_all_users_use_case,
        update_user_use_case,
        delete_user_use_case
    );
    builder.register_service(user_controller);
    debug!("Controlador de usuarios registrado");
    
    info!("Módulo de usuarios registrado correctamente");
    Ok(())
}

/// Registra todos los componentes relacionados con el dominio de usuarios
/// utilizando la implementación optimizada basada en SQLx para consultas
pub async fn register_with_sqlx(builder: &mut ContainerBuilder) -> Result<()> {
    debug!("Registrando componentes del módulo de usuarios con soporte SQLx");
    
    // Obtener pool de SQLx
    let sqlx_pool = crate::Infrastructure::Persistence::sqlx_database::get_default_pool().await?;
    let sqlx_pool_arc = Arc::new(sqlx_pool);
    
    // Registrar el mapper
    let user_mapper = Arc::new(UserMapper::new());
    builder.register_service(user_mapper.clone());
    
    // Registrar los repositorios
    let user_repository = Arc::new(
        UserRepositoryImpl::new()?
    );
    builder.register_service(user_repository.clone());
    
    // Usar SQLx para consultas (optimización)
    let user_query_repository = Arc::new(
        UserQueryRepositorySqlx::with_pool(sqlx_pool_arc.clone())
    );
    builder.register_service(user_query_repository.clone());
    
    let user_command_repository = Arc::new(
        UserCommandRepositoryImpl::new()?
    );
    builder.register_service(user_command_repository.clone());
    debug!("Repositorios de usuarios registrados (SQLx para consultas)");
    
    // Obtener el servicio de autenticación
    let auth_service = Arc::new(
        AuthServiceImpl::new()?
    );
    
    // Registrar caso de uso optimizado con SQLx
    let find_user_by_username_optimized_use_case = Arc::new(
        FindUserByUsernameOptimizedUseCase::new(
            user_query_repository.clone(),
            user_mapper.clone()
        )
    );
    builder.register_service(find_user_by_username_optimized_use_case.clone());
    debug!("Caso de uso optimizado de búsqueda por username registrado");
    
    // Registrar los casos de uso estándar
    let create_user_use_case = Arc::new(
        CreateUserUseCase::new(
            user_repository.clone(),
            auth_service.clone(),
            user_mapper.clone()
        )
    );
    builder.register_service(create_user_use_case.clone());
    
    let find_user_by_id_use_case = Arc::new(
        FindUserByIdUseCase::new(
            user_repository.clone(),
            user_mapper.clone()
        )
    );
    builder.register_service(find_user_by_id_use_case.clone());
    
    let find_all_users_use_case = Arc::new(
        FindAllUsersUseCase::new(
            user_repository.clone(),
            user_mapper.clone()
        )
    );
    builder.register_service(find_all_users_use_case.clone());
    
    let update_user_use_case = Arc::new(
        UpdateUserUseCase::new(
            user_repository.clone(),
            user_query_repository.clone(),
            user_mapper.clone(),
            auth_service.clone()
        )
    );
    builder.register_service(update_user_use_case.clone());
    
    let delete_user_use_case = Arc::new(
        DeleteUserUseCase::new(
            user_repository.clone()
        )
    );
    builder.register_service(delete_user_use_case.clone());
    debug!("Casos de uso estándar de usuarios registrados");
    
    // Registrar el controlador con la versión optimizada
    let user_controller = UserController::new(
        create_user_use_case,
        find_user_by_id_use_case,
        find_user_by_username_optimized_use_case, // Usar versión optimizada
        find_all_users_use_case,
        update_user_use_case,
        delete_user_use_case
    );
    builder.register_service(user_controller);
    debug!("Controlador de usuarios con soporte SQLx registrado");
    
    info!("Módulo de usuarios con soporte SQLx registrado correctamente");
    Ok(())
}