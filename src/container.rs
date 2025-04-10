use actix_web::web;
use std::sync::Arc;
use anyhow::{Result, Context}; // Usamos anyhow para un manejo de errores más sencillo

// Importaciones de implementaciones concretas y controladores
// Usamos 'crate::' para referirnos a los módulos definidos en src/lib.rs
use crate::infrastructure::repositories::user_repository_impl::UserRepositoryImpl;
use crate::infrastructure::auth::auth_service_impl::AuthServiceImpl;
use crate::application::use_cases::user::login::LoginUseCase;
use crate::presentation::api::controllers::auth_controller::AuthController;

// Estructura que contiene los datos compartidos de la aplicación.
// Se puede clonar porque todos sus miembros son `web::Data` (que internamente usa Arc).
#[derive(Clone)]
pub struct AppState {
    pub auth_controller_data: web::Data<AuthController>,
    // Aquí añadirías otros controladores o datos compartidos en el futuro
    // pub user_controller_data: web::Data<UserController>,
}

impl AppState {
    /// Construye el estado compartido de la aplicación, instanciando todas las dependencias.
    pub fn build() -> Result<Self> {
        // --- Instanciación de Dependencias ---

        // 1. Capa de Infraestructura (Repositorios y Servicios)
        let user_repo = Arc::new(
            UserRepositoryImpl::new()
                .context("Failed to create UserRepositoryImpl")?
        );
        let auth_service = Arc::new(
            AuthServiceImpl::new()
                .context("Failed to create AuthServiceImpl")?
        );

        // 2. Capa de Aplicación (Casos de Uso)
        // Se inyectan las dependencias de infraestructura necesarias.
        let login_use_case = Arc::new(
            LoginUseCase::new(user_repo.clone(), auth_service.clone())
        );
        // Aquí instanciarías otros casos de uso...
        // let create_user_use_case = Arc::new(CreateUserUseCase::new(user_repo.clone(), ...));

        // 3. Capa de Presentación (Controladores)
        // Se inyectan los casos de uso necesarios.
        let auth_controller = AuthController::new(login_use_case);
        // Aquí instanciarías otros controladores...
        // let user_controller = UserController::new(create_user_use_case, ...);

        // 4. Envolver controladores en web::Data para compartirlos
        let auth_controller_data = web::Data::new(auth_controller);
        // let user_controller_data = web::Data::new(user_controller);

        // 5. Construir y devolver el AppState
        Ok(AppState {
            auth_controller_data,
            // user_controller_data,
        })
    }
}