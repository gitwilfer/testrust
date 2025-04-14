use anyhow::Result;
// use std::sync::Arc;
use log::warn;

// Marcar el archivo como obsoleto
#[deprecated(
    since = "0.2.0",
    note = "Use factory::create_dependencies() o factory::create_dependencies_with_sqlx() en su lugar"
)]
pub use crate::factory::dependency_provider::AppDependencies;

#[deprecated(
    since = "0.2.0",
    note = "Use factory::create_dependencies() en su lugar"
)]
pub fn create_dependencies() -> Result<AppDependencies> {
    warn!("factory.rs está obsoleto. Por favor, use factory::create_dependencies() en su lugar");
    crate::factory::create_dependencies()
}

#[deprecated(
    since = "0.2.0",
    note = "Use factory::create_dependencies_with_sqlx() en su lugar"
)]
pub async fn create_dependencies_with_sqlx() -> Result<AppDependencies> {
    warn!("factory.rs está obsoleto. Por favor, use factory::create_dependencies_with_sqlx() en su lugar");
    crate::factory::create_dependencies_with_sqlx().await
}

// Mantener compatibilidad con estructuras existentes si es necesario