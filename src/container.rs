use actix_web::web;
use std::sync::Arc;
use anyhow::{Result, Context};

// Importaciones de implementaciones concretas y controladores
// Usamos 'crate::' para referirnos a los módulos definidos en src/lib.rs
use crate::Infrastructure::repositories::{UserRepositoryImpl, UserQueryRepositorySqlx};
use crate::Infrastructure::auth::AuthServiceImpl;
use crate::Infrastructure::Persistence::database;
use crate::Infrastructure::Persistence::sqlx_database;
use crate::Application::use_cases::user::login::LoginUseCase;
use crate::Application::use_cases::user::find_by_username_optimized::FindUserByUsernameOptimizedUseCase;
use crate::Presentation::api::controllers::{AuthController, UserController};
use crate::Application::mappers::UserMapper;
use crate::Infrastructure::monitoring::database_health_monitor::DatabaseHealthMonitor;
use crate::Presentation::api::controllers::health_controller::HealthController;

// Estructura que contiene los datos compartidos de la aplicación.
// Se puede clonar porque todos sus miembros son `web::Data` (que internamente usa Arc).
#[derive(Clone)]
pub struct AppState {
    pub auth_controller_data: web::Data<AuthController>,
    pub user_controller_data: web::Data<UserController>,
    pub health_controller_data: web::Data<HealthController>,
    // Aquí añadirías otros controladores o datos compartidos en el futuro
}

impl AppState {
    /// Construye el estado compartido de la aplicación, instanciando todas las dependencias.
    
    pub async fn build_with_sqlx() -> Result<Self> {
        // Inicializar SQLx
        let sqlx_pool = sqlx_database::get_default_pool().await
            .context("Failed to get SQLx pool")?;
        let sqlx_pool_arc = Arc::new(sqlx_pool);
        
        // Obtener pool de Diesel
        let diesel_pool = Arc::new(database::get_pool_from_connection());
        
        // Instanciar repositorios
        let user_repository = Arc::new(UserRepositoryImpl::new()?);
        let user_query_repository = Arc::new(UserQueryRepositorySqlx::with_pool(sqlx_pool_arc));
        let auth_service = Arc::new(AuthServiceImpl::new()?);
        let user_mapper = Arc::new(UserMapper::new());
        
        // NUEVO: Inicializar monitor de salud de base de datos
        // Verificar la salud cada 60 segundos
        let db_monitor = Arc::new(DatabaseHealthMonitor::new(60)); 
        // Iniciar el monitoreo en segundo plano
        db_monitor.start_monitoring();

        // Casos de uso
        let login_use_case = Arc::new(LoginUseCase::new(
            user_repository.clone(),
            auth_service.clone()
        ));
        
        // ⭐ Caso de uso optimizado con SQLx
        let find_user_by_username_use_case = Arc::new(FindUserByUsernameOptimizedUseCase::new(
            user_query_repository.clone(),
            user_mapper.clone()
        ));
        
        // Instanciar controladores

        // NUEVO: Crear health controller
        let health_controller = HealthController::new(db_monitor);
        let health_controller_data = web::Data::new(health_controller);

        let auth_controller = AuthController::new(login_use_case);
        let user_controller = UserController::new(
            Arc::new(crate::Application::use_cases::user::create::CreateUserUseCase::new(
                user_repository.clone(),
                auth_service.clone(),
                user_mapper.clone()
            )),
            Arc::new(crate::Application::use_cases::user::find_by_id::FindUserByIdUseCase::new(
                user_repository.clone(),
                user_mapper.clone()
            )),
            find_user_by_username_use_case,
            Arc::new(crate::Application::use_cases::user::find_all::FindAllUsersUseCase::new(
                user_repository.clone(),
                user_mapper.clone()
            )),
            Arc::new(crate::Application::use_cases::user::update::UpdateUserUseCase::new(
                user_repository.clone(),
                user_mapper.clone(),
                auth_service.clone()
            )),
            Arc::new(crate::Application::use_cases::user::delete::DeleteUserUseCase::new(
                user_repository.clone()
            ))
        );
        
        // Envolver controladores en web::Data para compartirlos
        let auth_controller_data = web::Data::new(auth_controller);
        let user_controller_data = web::Data::new(user_controller);
        
        // Construir y devolver el AppState
        Ok(AppState {
            auth_controller_data,
            user_controller_data,
            health_controller_data,
        })
    }


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
        
        // 3. Capa de Presentación (Controladores)
        // Se inyectan los casos de uso necesarios.
        let auth_controller = AuthController::new(login_use_case);
        
        // Crear un controlador de usuario vacío para la implementación básica
        let user_controller = UserController::new(
            Arc::new(crate::Application::use_cases::user::create::CreateUserUseCase::new(
                user_repo.clone(),
                auth_service.clone(),
                Arc::new(UserMapper::new())
            )),
            Arc::new(crate::Application::use_cases::user::find_by_id::FindUserByIdUseCase::new(
                user_repo.clone(),
                Arc::new(UserMapper::new())
            )),
            Arc::new(crate::Application::use_cases::user::find_by_username::FindUserByUsernameUseCase::new(
                user_repo.clone(),
                Arc::new(UserMapper::new())
            )),
            Arc::new(crate::Application::use_cases::user::find_all::FindAllUsersUseCase::new(
                user_repo.clone(),
                Arc::new(UserMapper::new())
            )),
            Arc::new(crate::Application::use_cases::user::update::UpdateUserUseCase::new(
                user_repo.clone(),
                Arc::new(UserMapper::new()),
                auth_service.clone()
            )),
            Arc::new(crate::Application::use_cases::user::delete::DeleteUserUseCase::new(
                user_repo.clone()
            ))
        );

        // Crear un monitor de salud básico
        let db_monitor = Arc::new(DatabaseHealthMonitor::new(60));
        db_monitor.start_monitoring();
        let health_controller = HealthController::new(db_monitor);

        // 4. Envolver controladores en web::Data para compartirlos
        let auth_controller_data = web::Data::new(auth_controller);
        let user_controller_data = web::Data::new(user_controller);
        let health_controller_data = web::Data::new(health_controller);

        // 5. Construir y devolver el AppState
        Ok(AppState {
            auth_controller_data,
            user_controller_data,
            health_controller_data,
        })
    }
}