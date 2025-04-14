use std::sync::Arc;
use anyhow::Result;
use log::{info, debug, trace};

use crate::Factory::dependency_provider::{DefaultDependencyProvider, DependencyProvider};
use crate::Application::use_cases::user::login::LoginUseCase;
use crate::Infrastructure::auth::AuthServiceImpl;
use crate::Infrastructure::repositories::{UserQueryRepositoryImpl, UserQueryRepositorySqlx};
use crate::Application::ports::repositories::{UserQueryRepository, AuthServicePort};

/// Registra todos los componentes relacionados con autenticación
pub fn register(provider: &mut DefaultDependencyProvider) -> Result<()> {
    debug!("Registrando componentes de autenticación...");
    
    // Registrar servicio de autenticación
    let auth_service = if provider.has::<dyn AuthServicePort>() {
        provider.get::<dyn AuthServicePort>().unwrap()
    } else {
        let service = Arc::new(AuthServiceImpl::new()?);
        provider.register::<dyn AuthServicePort>(service.clone());
        service
    };
    trace!("AuthServicePort registrado");
    
    // Obtener repositorio de consulta
    let user_query_repository = if provider.has::<dyn UserQueryRepository>() {
        provider.get::<dyn UserQueryRepository>().unwrap()
    } else {
        // Si no existe, crearlo
        let repository = Arc::new(UserQueryRepositoryImpl::new()?);
        provider.register::<dyn UserQueryRepository>(repository.clone());
        repository
    };
    
    // Registrar caso de uso de login
    let login_use_case = Arc::new(
        LoginUseCase::new(
            user_query_repository.clone(),
            auth_service.clone()
        )
    );
    provider.register::<LoginUseCase>(login_use_case);
    trace!("LoginUseCase registrado");
    
    info!("Componentes de autenticación registrados correctamente");
    Ok(())
}

/// Registra todos los componentes relacionados con autenticación con soporte SQLx
pub fn register_with_sqlx(provider: &mut DefaultDependencyProvider) -> Result<()> {
    debug!("Registrando componentes de autenticación con soporte SQLx...");
    
    // Verificar si ya existe un repositorio de consulta
    if provider.has::<dyn UserQueryRepository>() {
        debug!("UserQueryRepository ya registrado, usando existente");
        return register(provider);
    }
    
    // Verificar si se ha configurado un pool de SQLx
    let sqlx_pool = match provider.get_sqlx_pool() {
        Some(pool) => pool,
        None => {
            debug!("No se encontró pool SQLx en el proveedor, registrando componentes estándar");
            return register(provider);
        }
    };
    
    // Registrar servicio de autenticación
    let auth_service = if provider.has::<dyn AuthServicePort>() {
        provider.get::<dyn AuthServicePort>().unwrap()
    } else {
        let service = Arc::new(AuthServiceImpl::new()?);
        provider.register::<dyn AuthServicePort>(service.clone());
        service
    };
    
    // Crear repositorio de consulta optimizado con SQLx
    let user_query_repository = Arc::new(
        UserQueryRepositorySqlx::with_pool(sqlx_pool.clone())
    );
    provider.register::<dyn UserQueryRepository>(user_query_repository.clone());
    trace!("UserQueryRepository (SQLx) registrado");
    
    // Registrar caso de uso de login
    let login_use_case = Arc::new(
        LoginUseCase::new(
            user_query_repository.clone(),
            auth_service.clone()
        )
    );
    provider.register::<LoginUseCase>(login_use_case);
    trace!("LoginUseCase registrado");
    
    info!("Componentes de autenticación con soporte SQLx registrados correctamente");
    Ok(())
}