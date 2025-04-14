use std::sync::Arc;
use anyhow::Result;
use log::{info, debug};

use crate::Container::builder::ContainerBuilder;
use crate::Application::use_cases::user::login::LoginUseCase;
use crate::Infrastructure::auth::AuthServiceImpl;
// Importar los repositorios necesarios para PASARLOS como argumento
use crate::Infrastructure::repositories::{UserQueryRepositoryImpl, UserQueryRepositorySqlx};
use crate::Presentation::api::controllers::AuthController;
use sqlx::PgPool as SqlxPool;

// --- Structs para devolver grupos de dependencias ---
struct AuthServices {
    auth_service: Arc<AuthServiceImpl>,
}

struct AuthUseCases {
    login_use_case: Arc<LoginUseCase>,
}
// --- Fin Structs ---

pub struct AuthModule;

impl AuthModule {
    // --- Versión con Diesel ---
    pub fn register(builder: &mut ContainerBuilder) -> Result<()> {
        debug!("Registrando componentes del módulo de autenticación (Diesel)");

        // !! IMPORTANTE: Obtener UserQueryRepositoryImpl !!
        // Asumimos que user_module ya lo registró. Si get_service no existe,
        // este repo necesita ser pasado a `register` desde `modules/mod.rs`.
        // Comentado por ahora.
        // let user_query_repository = builder.get_service::<UserQueryRepositoryImpl>().expect("UserQueryRepositoryImpl not registered");
        // --- Alternativa Temporal: Crear instancia aquí ---
        let user_query_repository = Arc::new(UserQueryRepositoryImpl::new()?); // ¡OJO!
        debug!("UserQueryRepositoryImpl obtenido/creado para AuthModule (Diesel)");


        let services = Self::build_and_register_services(builder)?;
        let use_cases = Self::build_and_register_use_cases(builder, services.auth_service.clone(), user_query_repository)?; // Pasar dependencias
        Self::build_and_register_controller(builder, use_cases)?; // Pasar dependencias

        info!("Módulo de autenticación (Diesel) registrado correctamente");
        Ok(())
    }

    // Helper para Servicios (Diesel) - Devuelve instancia
    fn build_and_register_services(builder: &mut ContainerBuilder) -> Result<AuthServices> {
        let auth_service = Arc::new(AuthServiceImpl::new()?);
        builder.register_service(auth_service.clone());
        debug!("Servicio de autenticación registrado");
        Ok(AuthServices { auth_service }) // Devolver
    }

    // Helper para Casos de Uso (Diesel) - Recibe y devuelve instancias
    fn build_and_register_use_cases(
        builder: &mut ContainerBuilder,
        auth_service: Arc<AuthServiceImpl>, // Recibe AuthService
        user_query_repository: Arc<UserQueryRepositoryImpl>, // Recibe Repo
    ) -> Result<AuthUseCases> {
        let login_use_case = Arc::new(
            LoginUseCase::new(
                user_query_repository.clone(),
                auth_service.clone()
            )
        );
        builder.register_service(login_use_case.clone());
        debug!("Caso de uso de login (Diesel) registrado");
        Ok(AuthUseCases { login_use_case }) // Devolver
    }

    // Helper para Controlador (Diesel) - Recibe instancias
    fn build_and_register_controller(
        builder: &mut ContainerBuilder,
        use_cases: AuthUseCases, // Recibe casos de uso
    ) -> Result<()> {
        let auth_controller = Arc::new(AuthController::new(use_cases.login_use_case));
        // Usa register_arc_service si existe, sino register_service
        builder.register_arc_service(auth_controller);
        // builder.register_service(auth_controller); // Alternativa

        debug!("Controlador de autenticación (Diesel) registrado");
        Ok(())
    }


    // --- Versión con SQLx ---
    pub async fn register_with_sqlx(builder: &mut ContainerBuilder) -> Result<()> {
        debug!("Registrando componentes del módulo de autenticación (SQLx)");
        let sqlx_pool = crate::Infrastructure::Persistence::sqlx_database::get_default_pool().await?;
        let sqlx_pool_arc = Arc::new(sqlx_pool);

        // !! IMPORTANTE: Obtener UserQueryRepositorySqlx !!
        // Asumimos que user_module ya lo registró. Si get_service no existe,
        // este repo necesita ser pasado a `register_with_sqlx` desde `modules/mod.rs`.
        // Comentado por ahora.
        // let user_query_repository_sqlx = builder.get_service::<UserQueryRepositorySqlx>().expect("UserQueryRepositorySqlx not registered");
        // --- Alternativa Temporal: Crear instancia aquí ---
        let user_query_repository_sqlx = Arc::new(UserQueryRepositorySqlx::with_pool(sqlx_pool_arc.clone())); // ¡OJO!
        debug!("UserQueryRepositorySqlx obtenido/creado para AuthModule (SQLx)");


        let services = Self::build_and_register_services_sqlx(builder)?;
        let use_cases = Self::build_and_register_use_cases_sqlx(builder, services.auth_service.clone(), user_query_repository_sqlx)?; // Pasar dependencias
        Self::build_and_register_controller_sqlx(builder, use_cases)?; // Pasar dependencias

        info!("Módulo de autenticación (SQLx) registrado correctamente");
        Ok(())
    }

    // Helper para Servicios (SQLx) - Igual que Diesel, devuelve instancia
    fn build_and_register_services_sqlx(builder: &mut ContainerBuilder) -> Result<AuthServices> {
        Self::build_and_register_services(builder)
    }

    // Helper para Casos de Uso (SQLx) - Recibe y devuelve instancias
    fn build_and_register_use_cases_sqlx(
        builder: &mut ContainerBuilder,
        auth_service: Arc<AuthServiceImpl>, // Recibe AuthService
        user_query_repository_sqlx: Arc<UserQueryRepositorySqlx>, // Recibe Repo SQLx
    ) -> Result<AuthUseCases> {
        let login_use_case = Arc::new(
            LoginUseCase::new(
                user_query_repository_sqlx.clone(), // Usa la versión SQLx
                auth_service.clone()
            )
        );
        builder.register_service(login_use_case.clone());
        debug!("Caso de uso de login (SQLx) registrado");
        Ok(AuthUseCases { login_use_case }) // Devolver
    }

    // Helper para Controlador (SQLx) - Igual que Diesel
    fn build_and_register_controller_sqlx(
        builder: &mut ContainerBuilder,
        use_cases: AuthUseCases, // Recibe casos de uso
    ) -> Result<()> {
        Self::build_and_register_controller(builder, use_cases)
    }
}
