use std::sync::Arc;
use anyhow::Result;
use log::{info, debug};

use crate::Container::builder::ContainerBuilder;
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
use crate::Infrastructure::auth::AuthServiceImpl; // Importar para pasar como argumento
use crate::Presentation::api::controllers::UserController;
use sqlx::PgPool as SqlxPool;

// --- Structs para devolver grupos de dependencias ---
struct UserRepositoriesDiesel {
    user_repository: Arc<UserRepositoryImpl>,
    user_query_repository: Arc<UserQueryRepositoryImpl>,
    user_command_repository: Arc<UserCommandRepositoryImpl>,
}

struct UserRepositoriesSqlx {
    user_repository: Arc<UserRepositoryImpl>, // Diesel
    user_query_repository_sqlx: Arc<UserQueryRepositorySqlx>, // SQLx
    user_command_repository: Arc<UserCommandRepositoryImpl>, // Diesel
}

struct UserUseCasesDiesel {
    create_user_use_case: Arc<CreateUserUseCase>,
    find_user_by_id_use_case: Arc<FindUserByIdUseCase>,
    find_user_by_username_use_case: Arc<FindUserByUsernameUseCase>,
    find_all_users_use_case: Arc<FindAllUsersUseCase>,
    update_user_use_case: Arc<UpdateUserUseCase>,
    delete_user_use_case: Arc<DeleteUserUseCase>,
}

struct UserUseCasesSqlx {
    create_user_use_case: Arc<CreateUserUseCase>,
    find_user_by_id_use_case: Arc<FindUserByIdUseCase>,
    find_user_by_username_optimized_use_case: Arc<FindUserByUsernameOptimizedUseCase>, // Optimizado
    find_all_users_use_case: Arc<FindAllUsersUseCase>,
    update_user_use_case: Arc<UpdateUserUseCase>,
    delete_user_use_case: Arc<DeleteUserUseCase>,
}
// --- Fin Structs ---


pub struct UserModule;

impl UserModule {
    // --- Versión con Diesel ---
    pub fn register(builder: &mut ContainerBuilder) -> Result<()> {
        debug!("Registrando componentes del módulo de usuarios (Diesel)");

        // Obtener AuthService (asumiendo que auth_module ya lo registró)
        // !! ESTA ES LA ÚNICA LLAMADA A GET QUE PODRÍA QUEDAR, SI EL BUILDER LO PERMITE !!
        // Si `get_service` NO existe, AuthService necesita ser pasado a `register`
        // o manejado de otra forma (ej: construirlo aquí si es simple, o error).
        // Por ahora, lo comentamos y asumimos que se pasa o se maneja diferente.
        // let auth_service = builder.get_service::<AuthServiceImpl>().expect("AuthServiceImpl not registered");
        // --- Alternativa Temporal: Crear instancia aquí si es necesario y no se puede obtener ---
        let auth_service = Arc::new(AuthServiceImpl::new()?); // ¡OJO! Esto puede ser incorrecto si AuthServiceImpl tiene dependencias
        debug!("AuthService obtenido/creado para UserModule (Diesel)");

        let user_mapper = Self::build_and_register_mapper(builder)?;
        let repos = Self::build_and_register_repositories(builder)?;
        let use_cases = Self::build_and_register_use_cases(builder, user_mapper, repos, auth_service)?; // Pasar dependencias
        Self::build_and_register_controller(builder, use_cases)?; // Pasar dependencias

        info!("Módulo de usuarios (Diesel) registrado correctamente");
        Ok(())
    }

    // Helper para Mapper (Diesel) - Devuelve la instancia
    fn build_and_register_mapper(builder: &mut ContainerBuilder) -> Result<Arc<UserMapper>> {
        let user_mapper = Arc::new(UserMapper::new());
        builder.register_service(user_mapper.clone()); // Registrar
        debug!("Mapper de usuarios registrado");
        Ok(user_mapper) // Devolver
    }

    // Helper para Repositorios (Diesel) - Devuelve las instancias
    fn build_and_register_repositories(builder: &mut ContainerBuilder) -> Result<UserRepositoriesDiesel> {
        let user_repository = Arc::new(UserRepositoryImpl::new()?);
        builder.register_service(user_repository.clone());

        let user_query_repository = Arc::new(UserQueryRepositoryImpl::new()?);
        builder.register_service(user_query_repository.clone());

        let user_command_repository = Arc::new(UserCommandRepositoryImpl::new()?);
        builder.register_service(user_command_repository.clone());

        debug!("Repositorios de usuarios (Diesel) registrados");
        Ok(UserRepositoriesDiesel { // Devolver
            user_repository,
            user_query_repository,
            user_command_repository,
        })
    }

    // Helper para Casos de Uso (Diesel) - Recibe y devuelve instancias
    fn build_and_register_use_cases(
        builder: &mut ContainerBuilder,
        user_mapper: Arc<UserMapper>,
        repos: UserRepositoriesDiesel,
        auth_service: Arc<AuthServiceImpl>, // Recibe AuthService
    ) -> Result<UserUseCasesDiesel> {

        let create_user_use_case = Arc::new(
            CreateUserUseCase::new(
                repos.user_repository.clone(),
                auth_service.clone(),
                user_mapper.clone()
            )
        );
        builder.register_service(create_user_use_case.clone());

        let find_user_by_id_use_case = Arc::new(
            FindUserByIdUseCase::new(
                repos.user_repository.clone(),
                user_mapper.clone()
            )
        );
        builder.register_service(find_user_by_id_use_case.clone());

        let find_user_by_username_use_case = Arc::new(
            FindUserByUsernameUseCase::new(
                repos.user_repository.clone(),
                user_mapper.clone()
            )
        );
        builder.register_service(find_user_by_username_use_case.clone());

        let find_all_users_use_case = Arc::new(
            FindAllUsersUseCase::new(
                repos.user_repository.clone(),
                user_mapper.clone()
            )
        );
        builder.register_service(find_all_users_use_case.clone());

        let update_user_use_case = Arc::new(
            UpdateUserUseCase::new(
                repos.user_repository.clone(),
                repos.user_query_repository.clone(), // Usa el repo de consulta específico
                user_mapper.clone(),
                auth_service.clone()
            )
        );
        builder.register_service(update_user_use_case.clone());

        let delete_user_use_case = Arc::new(
            DeleteUserUseCase::new(
                repos.user_repository.clone()
            )
        );
        builder.register_service(delete_user_use_case.clone());

        debug!("Casos de uso de usuarios (Diesel) registrados");
        Ok(UserUseCasesDiesel { // Devolver
             create_user_use_case,
             find_user_by_id_use_case,
             find_user_by_username_use_case,
             find_all_users_use_case,
             update_user_use_case,
             delete_user_use_case,
        })
    }

    // Helper para Controlador (Diesel) - Recibe instancias
    fn build_and_register_controller(
        builder: &mut ContainerBuilder,
        use_cases: UserUseCasesDiesel, // Recibe casos de uso
    ) -> Result<()> {
        let user_controller = Arc::new(UserController::new(
            use_cases.create_user_use_case,
            use_cases.find_user_by_id_use_case,
            use_cases.find_user_by_username_use_case, // Usa la versión estándar aquí
            use_cases.find_all_users_use_case,
            use_cases.update_user_use_case,
            use_cases.delete_user_use_case
        ));
        // Usa register_arc_service si existe, sino register_service
        builder.register_arc_service(user_controller);
        // builder.register_service(user_controller); // Alternativa si register_arc_service no existe

        debug!("Controlador de usuarios (Diesel) registrado");
        Ok(())
    }


    // --- Versión con SQLx ---
    pub async fn register_with_sqlx(builder: &mut ContainerBuilder) -> Result<()> {
        debug!("Registrando componentes del módulo de usuarios (SQLx)");
        let sqlx_pool = crate::Infrastructure::Persistence::sqlx_database::get_default_pool().await?;
        let sqlx_pool_arc = Arc::new(sqlx_pool);

        // Obtener AuthService (ver nota en la versión Diesel)
        // let auth_service = builder.get_service::<AuthServiceImpl>().expect("AuthServiceImpl not registered");
        // --- Alternativa Temporal ---
        let auth_service = Arc::new(AuthServiceImpl::new()?); // ¡OJO!
        debug!("AuthService obtenido/creado para UserModule (SQLx)");


        let user_mapper = Self::build_and_register_mapper_sqlx(builder)?;
        let repos = Self::build_and_register_repositories_sqlx(builder, sqlx_pool_arc.clone())?;
        let use_cases = Self::build_and_register_use_cases_sqlx(builder, user_mapper, repos, auth_service)?; // Pasar dependencias
        Self::build_and_register_controller_sqlx(builder, use_cases)?; // Pasar dependencias

        info!("Módulo de usuarios (SQLx) registrado correctamente");
        Ok(())
    }

    // Helper para Mapper (SQLx) - Igual que Diesel, devuelve instancia
    fn build_and_register_mapper_sqlx(builder: &mut ContainerBuilder) -> Result<Arc<UserMapper>> {
        Self::build_and_register_mapper(builder)
    }

    // Helper para Repositorios (SQLx) - Devuelve instancias
    fn build_and_register_repositories_sqlx(
        builder: &mut ContainerBuilder,
        sqlx_pool_arc: Arc<SqlxPool>
    ) -> Result<UserRepositoriesSqlx> {
        let user_repository = Arc::new(UserRepositoryImpl::new()?);
        builder.register_service(user_repository.clone());

        let user_query_repository_sqlx = Arc::new(
            UserQueryRepositorySqlx::with_pool(sqlx_pool_arc.clone())
        );
        builder.register_service(user_query_repository_sqlx.clone());

        let user_command_repository = Arc::new(UserCommandRepositoryImpl::new()?);
        builder.register_service(user_command_repository.clone());

        debug!("Repositorios de usuarios (SQLx para consultas) registrados");
        Ok(UserRepositoriesSqlx { // Devolver
            user_repository,
            user_query_repository_sqlx,
            user_command_repository,
        })
    }

    // Helper para Casos de Uso (SQLx) - Recibe y devuelve instancias
    fn build_and_register_use_cases_sqlx(
        builder: &mut ContainerBuilder,
        user_mapper: Arc<UserMapper>,
        repos: UserRepositoriesSqlx, // Recibe repos SQLx
        auth_service: Arc<AuthServiceImpl>, // Recibe AuthService
    ) -> Result<UserUseCasesSqlx> {

        let find_user_by_username_optimized_use_case = Arc::new(
            FindUserByUsernameOptimizedUseCase::new(
                repos.user_query_repository_sqlx.clone(), // Usa la versión SQLx
                user_mapper.clone()
            )
        );
        builder.register_service(find_user_by_username_optimized_use_case.clone());
        debug!("Caso de uso optimizado de búsqueda por username (SQLx) registrado");

        let create_user_use_case = Arc::new(
            CreateUserUseCase::new(
                repos.user_repository.clone(),
                auth_service.clone(),
                user_mapper.clone()
            )
        );
        builder.register_service(create_user_use_case.clone());

        let find_user_by_id_use_case = Arc::new(
            FindUserByIdUseCase::new(
                repos.user_repository.clone(),
                user_mapper.clone()
            )
        );
        builder.register_service(find_user_by_id_use_case.clone());

        let find_all_users_use_case = Arc::new(
            FindAllUsersUseCase::new(
                repos.user_repository.clone(),
                user_mapper.clone()
            )
        );
        builder.register_service(find_all_users_use_case.clone());

        let update_user_use_case = Arc::new(
            UpdateUserUseCase::new(
                repos.user_repository.clone(),
                repos.user_query_repository_sqlx.clone(), // Usa la versión SQLx
                user_mapper.clone(),
                auth_service.clone()
            )
        );
        builder.register_service(update_user_use_case.clone());

        let delete_user_use_case = Arc::new(
            DeleteUserUseCase::new(
                repos.user_repository.clone()
            )
        );
        builder.register_service(delete_user_use_case.clone());

        debug!("Casos de uso estándar de usuarios (con consulta SQLx) registrados");
        Ok(UserUseCasesSqlx { // Devolver
            create_user_use_case,
            find_user_by_id_use_case,
            find_user_by_username_optimized_use_case, // Devuelve el optimizado
            find_all_users_use_case,
            update_user_use_case,
            delete_user_use_case,
        })
    }

     // Helper para Controlador (SQLx) - Recibe instancias
    fn build_and_register_controller_sqlx(
        builder: &mut ContainerBuilder,
        use_cases: UserUseCasesSqlx, // Recibe casos de uso SQLx
    ) -> Result<()> {
        let user_controller = Arc::new(UserController::new(
            use_cases.create_user_use_case,
            use_cases.find_user_by_id_use_case,
            use_cases.find_user_by_username_optimized_use_case, // Inyectar versión optimizada
            use_cases.find_all_users_use_case,
            use_cases.update_user_use_case,
            use_cases.delete_user_use_case
        ));
        // Usa register_arc_service si existe, sino register_service
        builder.register_arc_service(user_controller);
        // builder.register_service(user_controller); // Alternativa

        debug!("Controlador de usuarios (SQLx) registrado");
        Ok(())
    }
}
