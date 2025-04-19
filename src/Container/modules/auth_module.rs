use std::sync::Arc;
use anyhow::Result;
use log::{info, debug};

use crate::Container::builder::ContainerBuilder;
// Importar la struct concreta
use crate::Application::use_cases::user::login::LoginUseCase;
// Reintroducir la importación del trait explícitamente
use crate::Application::use_cases::traits::LoginUseCase as LoginUseCaseTrait;
use crate::Application::ports::driven::repositories::UserQueryRepository;
use crate::Application::ports::driven::AuthServicePort;
use crate::Infrastructure::auth::AuthServiceImpl;
use crate::Presentation::api::controllers::AuthController;

// Usar el alias del trait importado
struct AuthUseCases { login_use_case: Arc<dyn LoginUseCaseTrait> }

pub struct AuthModule;

impl AuthModule {
    pub fn register(builder: &mut ContainerBuilder) -> Result<()> {
        debug!("Registrando componentes del módulo de autenticación...");

        // --- Obtener Dependencias Registradas ---
        let user_query_repository = builder.registry().get_arc::<dyn UserQueryRepository>()
            .expect("UserQueryRepository not registered. Ensure RepositoryModule runs before AuthModule.");

        // --- Obtener/Registrar AuthServicePort ---
        let auth_service = if let Some(svc) = builder.registry().get_arc::<dyn AuthServicePort>() {
            svc
        } else {
            debug!("AuthServicePort no encontrado, registrando ahora...");
            let svc = Arc::new(AuthServiceImpl::new()?);
            builder.register_arc_service::<dyn AuthServicePort>(svc.clone());
            svc
        };

        let use_cases = Self::build_and_register_use_cases(
            builder,
            auth_service,
            user_query_repository
        )?;
        Self::build_and_register_controller(builder, use_cases)?;

        info!("Módulo de autenticación registrado correctamente");
        Ok(())
    }

    fn build_and_register_use_cases(
        builder: &mut ContainerBuilder,
        auth_service: Arc<dyn AuthServicePort>,
        user_query_repository: Arc<dyn UserQueryRepository>,
    ) -> Result<AuthUseCases> {
        // Cambiado: Usar la struct concreta LoginUseCase
        let login_use_case_impl = Arc::new(
            LoginUseCase::new(
                user_query_repository.clone(),
                auth_service.clone()
            )
        );
        // Usar el alias del trait importado al registrar
        builder.register_arc_service::<dyn LoginUseCaseTrait>(login_use_case_impl.clone());
        debug!("Caso de uso de login registrado");
        // Devolver la implementación concreta, pero el tipo del campo es Arc<dyn Trait>
        Ok(AuthUseCases { login_use_case: login_use_case_impl })
    }

    fn build_and_register_controller(
        builder: &mut ContainerBuilder,
        use_cases: AuthUseCases,
    ) -> Result<()> {
        // Pasar el Arc<dyn Trait> al controlador
        let auth_controller = Arc::new(AuthController::new(use_cases.login_use_case));
        builder.register_arc_service(auth_controller); // Registrar tipo concreto
        debug!("Controlador de autenticación registrado");
        Ok(())
    }
}
