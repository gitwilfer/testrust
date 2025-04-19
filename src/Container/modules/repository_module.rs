use crate::Infrastructure::repositories::{ // Implementaciones
    UserQueryRepositorySqlx,
    LogicalEntityQueryRepositoryImpl,
    DataTypeQueryRepositoryImpl,
    // Añadir otras implementaciones de consulta si existen
};
// --- CORREGIDO: Usar ruta completa o 'super::super::ports' ---
use crate::Application::ports::driven::repositories::{ // Traits
    UserQueryRepository,
    LogicalEntityQueryRepository,
    DataTypeQueryRepository,
    // Añadir otros traits de consulta si existen
};

use crate::Container::builder::ContainerBuilder;
use std::sync::Arc;
use sqlx::PgPool as SqlxPool; // Alias consistente
use anyhow::Result;
use log::debug;

pub async fn register_repository_dependencies(builder: &mut ContainerBuilder) -> Result<()> {
    debug!("Registrando repositorios de consulta (SQLx)...");
    let sqlx_pool = builder.registry().get_arc::<SqlxPool>()
        .expect("SQLx Pool not registered in Container");

    // --- User ---
    let user_query_repo = Arc::new(UserQueryRepositorySqlx::with_pool(sqlx_pool.clone()));
    builder.register_arc_service::<dyn UserQueryRepository>(user_query_repo);
    debug!("UserQueryRepository (SQLx) registrado.");

    // --- Logical Entity ---
    let le_query_repo = Arc::new(LogicalEntityQueryRepositoryImpl::with_pool(sqlx_pool.clone()));
    builder.register_arc_service::<dyn LogicalEntityQueryRepository>(le_query_repo);
    debug!("LogicalEntityQueryRepository (SQLx) registrado.");

    // --- DataType ---
    let dt_query_repo = Arc::new(DataTypeQueryRepositoryImpl::with_pool(sqlx_pool.clone()));
    builder.register_arc_service::<dyn DataTypeQueryRepository>(dt_query_repo);
    debug!("DataTypeQueryRepository (SQLx) registrado.");

    // --- Registrar otros repositorios de consulta aquí ---

    Ok(())
}
