// @generated automatically by Diesel CLI.

diesel::table! {
    usuarios (idx_usuario) {
        idx_usuario -> Uuid,
        usuario -> Text,
        nombre -> Text,
        apellido -> Text,
        correo_electronico -> Text,
        contraseña -> Text,
        creado_por -> Nullable<Uuid>,  // Cambiado de Text a Uuid
        fecha_creacion -> Timestamp,
        modificado_por -> Nullable<Uuid>,  // Cambiado de Text a Uuid
        fecha_modificacion -> Nullable<Timestamp>,
        status -> Int2,
    }
}

// Alias para hacer más sencilla la referencia
pub mod users {
    pub use super::usuarios::dsl::*;
    
    // Define el alias table para que schema::users::table funcione
    pub const table: super::usuarios::table = super::usuarios::table;
}