// e:\work\cursos\anyB\src\Container\modules\controller_module.rs
// *** CORREGIDO: Import de CreateEntityWithAttributesUseCase ***

use crate::Container::builder::ContainerBuilder;
use crate::Presentation::api::controllers::{
    AuthController, UserController, HealthController, LogicalEntityController
};
// --- Importar Traits de Casos de Uso ---
use crate::Application::use_cases::traits::{ // Traits de User/Auth
    LoginUseCase, CreateUserUseCase, FindUserByIdUseCase, FindUserByUsernameUseCase,
    FindAllUsersUseCase, UpdateUserUseCase, DeleteUserUseCase,
    // Añadir otros traits generales si existen
};
// --- CORREGIDO: Importar trait de Logical Entity desde su módulo ---
use crate::Application::use_cases::logical_entities::CreateEntityWithAttributesUseCase;
// -----------------------------------------------------------------
use std::sync::Arc;
use anyhow::Result;
use log::debug;

pub async fn register_controller_dependencies(builder: &mut ContainerBuilder) -> Result<()> {
    debug!("Registrando controladores...");

    // --- Obtener Casos de Uso (por Trait) ---
    let login_uc = builder.registry().get_arc::<dyn LoginUseCase>()
        .expect("LoginUseCase not registered.");
    let create_user_uc = builder.registry().get_arc::<dyn CreateUserUseCase>()
        .expect("CreateUserUseCase not registered.");
    let find_user_by_id_uc = builder.registry().get_arc::<dyn FindUserByIdUseCase>()
        .expect("FindUserByIdUseCase not registered.");
    let find_user_by_username_uc = builder.registry().get_arc::<dyn FindUserByUsernameUseCase>()
        .expect("FindUserByUsernameUseCase not registered.");
    let find_all_users_uc = builder.registry().get_arc::<dyn FindAllUsersUseCase>()
        .expect("FindAllUsersUseCase not registered.");
    let update_user_uc = builder.registry().get_arc::<dyn UpdateUserUseCase>()
        .expect("UpdateUserUseCase not registered.");
    let delete_user_uc = builder.registry().get_arc::<dyn DeleteUserUseCase>()
        .expect("DeleteUserUseCase not registered.");

    // Obtener el trait correcto (la ruta de import ahora es correcta)
    let create_le_uc = builder.registry().get_arc::<dyn CreateEntityWithAttributesUseCase>()
        .expect("CreateEntityWithAttributesUseCase not registered.");
    // ------------------------------------------
    // ... obtener otros casos de uso ...
    // ------------------------------------

    // --- Construir y Registrar Controladores ---
    let auth_controller = Arc::new(AuthController::new(login_uc));
    builder.register_arc_service(auth_controller);
    debug!("AuthController registrado.");

    let user_controller = Arc::new(UserController::new(
        create_user_uc,
        find_user_by_id_uc,
        find_user_by_username_uc,
        find_all_users_uc,
        update_user_uc,
        delete_user_uc,
    ));
    builder.register_arc_service(user_controller);
    debug!("UserController registrado.");

    // Pasar el trait correcto al constructor
    let le_controller = Arc::new(LogicalEntityController::new(create_le_uc /*, otros LE UCs */));
    builder.register_arc_service(le_controller);
    debug!("LogicalEntityController registrado.");

    // Health Controller
    let db_monitor = builder.registry().get_arc::<crate::Infrastructure::monitoring::DatabaseHealthMonitor>()
        .expect("DatabaseHealthMonitor not registered.");
    let health_controller = Arc::new(HealthController::new(db_monitor));
    builder.register_arc_service(health_controller);
    debug!("HealthController registrado.");
    // ---------------------------------------

    Ok(())
}
