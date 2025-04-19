use crate::Infrastructure::Persistence::unit_of_work::DieselAsyncUnitOfWork;

use crate::Application::ports::unit_of_work::UnitOfWork;
use crate::Container::builder::ContainerBuilder;
use std::sync::Arc;
use anyhow::Result;
use log::debug;

// --- Importar tipos de Pools ---
use diesel_async::pooled_connection::bb8::Pool as AsyncDieselPool;
use diesel_async::AsyncPgConnection;
use sqlx::PgPool as SqlxPool;

// --- Importar funciones de obtención de pools ---
use crate::Infrastructure::Persistence::connection_pools::{
    get_default_async_pool,
    get_default_sqlx_pool,
};

pub async fn register_database_dependencies(builder: &mut ContainerBuilder) -> Result<()> {
    debug!("Registrando dependencias de base de datos (Pools y UoW)...");

    // --- Obtener/Registrar Pool Diesel Async ---
    let diesel_async_pool: Arc<AsyncDieselPool<AsyncPgConnection>> = get_default_async_pool()
        .expect("Failed to get default async diesel pool. Ensure initialize_pools was called.");
    builder.register_arc_service(diesel_async_pool.clone());
    debug!("Pool Diesel Async registrado.");

    // --- Obtener/Registrar Pool SQLx ---
    let sqlx_pool: Arc<SqlxPool> = get_default_sqlx_pool().await
         .expect("Failed to get default SQLx pool. Ensure initialize_pools was called.");
    builder.register_arc_service(sqlx_pool.clone());
    debug!("Pool SQLx registrado.");

    // --- Registrar UnitOfWork ---
    // Asegúrate que DieselAsyncUnitOfWork::new acepte Arc<...>
    let uow = Arc::new(DieselAsyncUnitOfWork::new(diesel_async_pool, sqlx_pool));
    builder.register_arc_service::<dyn UnitOfWork>(uow);
    debug!("UnitOfWork registrada.");

    Ok(())
}
