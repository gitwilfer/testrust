// @generated automatically by Diesel CLI.

diesel::table! {
    usuarios (idx_usuario) {
        idx_usuario -> Uuid,
        usuario -> Text,
        nombre -> Text,
        apellido -> Text,
        correo_electronico -> Text,
        password_hash -> Text,
        creado_por -> Nullable<Text>,
        fecha_creacion -> Timestamp,
        modificado_por -> Nullable<Text>,
        fecha_modificacion -> Nullable<Timestamp>,
        status -> Int2,
    }
}
