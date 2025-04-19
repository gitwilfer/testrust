pub mod repositories;
// pub mod other_driven_ports; // Si tienes otros (ej: email, notificaciones)

// --- Auth Service (MOVIDO AQUÍ) ---
pub mod auth_service; // <-- Descomentar si creaste el archivo auth_service.rs
pub use auth_service::AuthServicePort;