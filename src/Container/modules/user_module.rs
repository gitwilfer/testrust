use std::sync::Arc;
use anyhow::Result;
use log::{info, debug};

use crate::Container::builder::ContainerBuilder;
use crate::Application::mappers::UserMapper;
use crate::Application::use_cases::user::create::CreateUserUseCaseImpl;
use crate::Application::use_cases::user::find_by_id::FindUserByIdUseCaseImpl;
use crate::Application::use_cases::user::find_by_username_optimized::FindUserByUsernameOptimizedUseCase;
use crate::Application::use_cases::user::find_all::FindAllUsersUseCaseImpl;
use crate::Application::use_cases::user::update::UpdateUserUseCaseImpl;
use crate::Application::use_cases::user::delete::DeleteUserUseCaseImpl;
use crate::Application::use_cases::traits::{
    CreateUserUseCase, FindUserByIdUseCase, FindUserByUsernameUseCase,
    FindAllUsersUseCase, UpdateUserUseCase, DeleteUserUseCase,
};
use crate::Application::ports::driven::repositories::{
    UserQueryRepository, UserCommandRepository
};
use crate::Application::ports::driven::AuthServicePort;
use crate::Application::ports::unit_of_work::UnitOfWork; // Importar UoW
use crate::Infrastructure::repositories::UserCommandRepositoryImpl;
use crate::Presentation::api::controllers::UserController;

// Estructura para agrupar los casos de uso (para pasar al controlador)
struct UserUseCases {
    create_user: Arc<dyn CreateUserUseCase>,
    find_by_id: Arc<dyn FindUserByIdUseCase>,
    find_by_username: Arc<dyn FindUserByUsernameUseCase>,
    find_all: Arc<dyn FindAllUsersUseCase>,
    update_user: Arc<dyn UpdateUserUseCase>,
    delete_user: Arc<dyn DeleteUserUseCase>,
}

pub struct UserModule;

impl UserModule {
    pub fn register(builder: &mut ContainerBuilder) -> Result<()> {
        debug!("Registrando componentes del módulo de usuarios...");

        // --- Obtener Dependencias Registradas ---
        let auth_service = builder.registry().get_arc::<dyn AuthServicePort>()
            .expect("AuthServicePort not registered. Ensure AuthModule runs before UserModule.");
        let user_query_repository = builder.registry().get_arc::<dyn UserQueryRepository>()
            .expect("UserQueryRepository not registered. Ensure RepositoryModule runs before UserModule.");
        let unit_of_work = builder.registry().get_arc::<dyn UnitOfWork>() // Obtener UoW
            .expect("UnitOfWork not registered. Ensure DatabaseModule runs before UserModule.");
        // ---------------------------------------

        let user_command_repository = if let Some(repo) = builder.registry().get_arc::<dyn UserCommandRepository>() {
            repo
        } else {
            debug!("UserCommandRepository no encontrado, registrando ahora...");
            let repo = Arc::new(UserCommandRepositoryImpl::new());
            builder.register_arc_service::<dyn UserCommandRepository>(repo.clone());
            repo
        };

        let user_mapper = Self::build_and_register_mapper(builder)?;
        let use_cases = Self::build_and_register_use_cases(
            builder,
            user_mapper,
            user_query_repository,
            user_command_repository,
            auth_service,
            unit_of_work // Pasar UoW
        )?;
        Self::build_and_register_controller(builder, use_cases)?;

        info!("Módulo de usuarios registrado correctamente");
        Ok(())
    }

    fn build_and_register_mapper(builder: &mut ContainerBuilder) -> Result<Arc<UserMapper>> { /* ... */ }

    fn build_and_register_use_cases(
        builder: &mut ContainerBuilder,
        user_mapper: Arc<UserMapper>,
        user_query_repository: Arc<dyn UserQueryRepository>,
        user_command_repository: Arc<dyn UserCommandRepository>,
        auth_service: Arc<dyn AuthServicePort>,
        unit_of_work: Arc<dyn UnitOfWork>, // Recibir UoW
    ) -> Result<UserUseCases> {

        // Crear Implementaciones inyectando dependencias (incluyendo UoW)
        let create_user_use_case_impl = Arc::new(
            CreateUserUseCaseImpl::new(
                user_command_repository.clone(),
                user_query_repository.clone(),
                auth_service.clone(),
                unit_of_work.clone(), // Inyectar UoW
                user_mapper.clone()
            )
        );
        builder.register_arc_service::<dyn CreateUserUseCase>(create_user_use_case_impl.clone());

        let find_user_by_id_use_case_impl = Arc::new(
            FindUserByIdUseCaseImpl::new(user_query_repository.clone(), user_mapper.clone())
        );
        builder.register_arc_service::<dyn FindUserByIdUseCase>(find_user_by_id_use_case_impl.clone());

        // Asumiendo que FindUserByUsernameOptimizedUseCaseImpl también necesita query_repo y mapper
        let find_user_by_username_optimized_use_case_impl = Arc::new(
            FindUserByUsernameOptimizedUseCase::new(user_query_repository.clone(), user_mapper.clone())
        );
        builder.register_arc_service::<dyn FindUserByUsernameUseCase>(find_user_by_username_optimized_use_case_impl.clone());

        let find_all_users_use_case_impl = Arc::new(
            FindAllUsersUseCaseImpl::new(user_query_repository.clone(), user_mapper.clone())
        );
        builder.register_arc_service::<dyn FindAllUsersUseCase>(find_all_users_use_case_impl.clone());

        let update_user_use_case_impl = Arc::new(
            UpdateUserUseCaseImpl::new(
                user_command_repository.clone(),
                user_query_repository.clone(),
                auth_service.clone(),
                unit_of_work.clone(), // Inyectar UoW
                user_mapper.clone()
            )
        );
        builder.register_arc_service::<dyn UpdateUserUseCase>(update_user_use_case_impl.clone());

        let delete_user_use_case_impl = Arc::new(
            DeleteUserUseCaseImpl::new(
                user_command_repository.clone(),
                unit_of_work.clone() // Inyectar UoW
            )
        );
        builder.register_arc_service::<dyn DeleteUserUseCase>(delete_user_use_case_impl.clone());

        debug!("Casos de uso de usuarios registrados");
        Ok(UserUseCases {
            create_user: create_user_use_case_impl,
            find_by_id: find_user_by_id_use_case_impl,
            find_by_username: find_user_by_username_optimized_use_case_impl,
            find_all: find_all_users_use_case_impl,
            update_user: update_user_use_case_impl,
            delete_user: delete_user_use_case_impl,
        })
    }

    fn build_and_register_controller(
        builder: &mut ContainerBuilder,
        use_cases: UserUseCases,
    ) -> Result<()> {
        // TODO: Implementar la lógica real para registrar el UserController // (Mantengo el TODO por si acaso)
        let user_controller = Arc::new(UserController::new(
            use_cases.create_user,
            use_cases.find_by_id,
            use_cases.find_by_username,
            use_cases.find_all,
            use_cases.update_user,
            use_cases.delete_user,
        ));
        // Registrar el tipo concreto UserController, ya que AppState lo espera así.
        builder.register_arc_service(user_controller);
        debug!("Controlador de usuarios registrado"); // Actualizar mensaje debug
        Ok(())
    }
}
