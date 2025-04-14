use anyhow::Result;
use std::sync::Arc;
use log::{info, warn};

// Marcar el archivo como obsoleto
#[deprecated(
    since = "0.2.0",
    note = "Use el módulo container::build() o container::build_with_sqlx() en su lugar"
)]
pub use crate::container::app_state::AppState;

#[deprecated(
    since = "0.2.0",
    note = "Use container::build() en su lugar"
)]
pub async fn build() -> Result<AppState> {
    warn!("container.rs está obsoleto. Por favor, use container::build() en su lugar");
    crate::container::build().await
}

#[deprecated(
    since = "0.2.0",
    note = "Use container::build_with_sqlx() en su lugar"
)]
pub async fn build_with_sqlx() -> Result<AppState> {
    warn!("container.rs está obsoleto. Por favor, use container::build_with_sqlx() en su lugar");
    crate::container::build_with_sqlx().await
}

// Si existían otras estructuras o funciones en container.rs,
// se pueden marcar como obsoletas o mover al nuevo módulo según corresponda