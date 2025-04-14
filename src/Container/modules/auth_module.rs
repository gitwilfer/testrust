use std::sync::Arc;
use anyhow::Result;
use log::{info, debug};

use crate::Container::builder::ContainerBuilder;
use crate::Application::use_cases::user::login::LoginUseCase;
use crate::Infrastructure::auth::AuthServiceImpl;
use crate::Infrastructure::repositories::{UserQueryRepositoryImpl, UserQueryRepositorySqlx};
use crate::Presentation::api::controllers::AuthController;

/// Registra todos los componentes relacionados con la autenticación
/// utilizando la implementación estándar basada en Diesel
pub fn register(builder: &mut ContainerBuilder) -> Result<()> {
    debug!("Registrando componentes del módulo de autenticación");
    
    // Registrar servicio de autenticación
    let auth_service = Arc::new(
        AuthServiceImpl::new()?
    );
    builder.register_service(auth_service.clone());
    debug!("Servicio de autenticación registrado");
    
    // Crear repositorio de consulta
    // En un sistema más avanzado, lo obtendríamos del registro
    let user_query_repository = Arc::new(
        UserQueryRepositoryImpl::new()?
    );
    
    // Crear caso de uso de login
    let login_use_case = Arc::new(
        LoginUseCase::new(
            user_query_repository.clone(),
            auth_service.clone()
        )
    );
    builder.register_service(login_use_case.clone());
    debug!("Caso de uso de login registrado");
    
    // Registrar controlador de autenticación
    let auth_controller = Arc::new(AuthController::new(login_use_case)); // Envolver en Arc
    builder.register_arc_service(auth_controller); // Usar register_arc_service
    debug!("Controlador de autenticación registrado");
    
    info!("Módulo de autenticación registrado correctamente");
    Ok(())
}

/// Registra todos los componentes relacionados con la autenticación
/// utilizando la implementación optimizada basada en SQLx para consultas
pub async fn register_with_sqlx(builder: &mut ContainerBuilder) -> Result<()> {
    debug!("Registrando componentes del módulo de autenticación con soporte SQLx");
    
    // Registrar servicio de autenticación
    let auth_service = Arc::new(
        AuthServiceImpl::new()?
    );
    builder.register_service(auth_service.clone());
    
    // Obtener pool de SQLx
    let sqlx_pool = crate::Infrastructure::Persistence::sqlx_database::get_default_pool().await?;
    let sqlx_pool_arc = Arc::new(sqlx_pool);
    
    // Crear repositorio de consulta optimizado con SQLx
    let user_query_repository = Arc::new(
        UserQueryRepositorySqlx::with_pool(sqlx_pool_arc.clone())
    );
    debug!("Repositorio de consulta SQLx para autenticación creado");
    
    // Crear caso de uso de login
    let login_use_case = Arc::new(
        LoginUseCase::new(
            user_query_repository.clone(),
            auth_service.clone()
        )
    );
    builder.register_service(login_use_case.clone());
    debug!("Caso de uso de login con repositorio SQLx registrado");
    
    // Registrar controlador de autenticación
    let auth_controller = Arc::new(AuthController::new(login_use_case)); // Envolver en Arc
    builder.register_arc_service(auth_controller); // Usar register_arc_service
    debug!("Controlador de autenticación registrado");
    
    info!("Módulo de autenticación con soporte SQLx registrado correctamente");
    Ok(())
}