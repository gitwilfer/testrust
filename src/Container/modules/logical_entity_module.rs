use std::sync::Arc;
use anyhow::Result;
use log::{info, debug};

use crate::Container::builder::ContainerBuilder;
// --- Traits y Structs Necesarias ---
use crate::Application::ports::driven::repositories::{
    LogicalEntityCommandRepository, LogicalEntityQueryRepository
};
use crate::Application::ports::unit_of_work::UnitOfWork;
use crate::Application::use_cases::logical_entity::create::{ // Asumiendo esta ruta
    CreateLogicalEntityUseCaseImpl, CreateLogicalEntityUseCase
};
use crate::Infrastructure::repositories::LogicalEntityCommandRepositoryImpl; // ZST
use crate::Presentation::api::controllers::LogicalEntityController; // Controlador

pub struct LogicalEntityModule;

impl LogicalEntityModule {
    pub fn register(builder: &mut ContainerBuilder) -> Result<()> {
        debug!("Registrando componentes del módulo de Logical Entity...");

        // --- Obtener Dependencias ---
        let le_query_repository = builder.registry().get_arc::<dyn LogicalEntityQueryRepository>()
            .expect("LogicalEntityQueryRepository not registered.");
        let unit_of_work = builder.registry().get_arc::<dyn UnitOfWork>()
            .expect("UnitOfWork not registered.");
        // --------------------------

        // --- Registrar LE Command Repo (ZST) ---
        let le_command_repository = if let Some(repo) = builder.registry().get_arc::<dyn LogicalEntityCommandRepository>() {
            repo
        } else {
            debug!("LogicalEntityCommandRepository no encontrado, registrando ahora...");
            let repo = Arc::new(LogicalEntityCommandRepositoryImpl::new());
            builder.register_arc_service::<dyn LogicalEntityCommandRepository>(repo.clone());
            repo
        };
        // -------------------------------------

        // --- Registrar Caso de Uso ---
        let create_le_use_case_impl = Arc::new(
            CreateLogicalEntityUseCaseImpl::new(
                le_command_repository.clone(),
                le_query_repository.clone(), // Para validación de nombre, etc.
                unit_of_work.clone() // Inyectar UoW
            )
        );
        builder.register_arc_service::<dyn CreateLogicalEntityUseCase>(create_le_use_case_impl.clone());
        debug!("CreateLogicalEntityUseCase registrado.");


        // --- Registrar Controlador ---
        let le_controller = Arc::new(
            LogicalEntityController::new(create_le_use_case_impl /*, otros casos de uso LE */)
        );
        builder.register_arc_service(le_controller);
        debug!("LogicalEntityController registrado.");
        // ---------------------------

        info!("Módulo de Logical Entity registrado correctamente.");
        Ok(())
    }
}
