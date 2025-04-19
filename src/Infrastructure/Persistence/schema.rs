// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password_hash -> Text,
        created_by -> Nullable<Uuid>,
        created_at -> Timestamp, // O Timestamptz, ajusta según tu BD
        updated_by -> Nullable<Uuid>,
        updated_at -> Nullable<Timestamp>, // O Timestamptz
        status -> Int2,
    }
}

diesel::table! {
    logical_entities (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        assign_view -> Nullable<Text>,
        created_by -> Nullable<Uuid>, // Asume referencia a users.id
        created_at -> Timestamptz,
        updated_by -> Nullable<Uuid>, // Asume referencia a users.id
        updated_at -> Nullable<Timestamptz>,
        status -> Int2,
    }
}

diesel::table! {
    // Definición placeholder para data_types, ajusta si es diferente
    data_types (id) {
        id -> Uuid,
        name -> Text, // Asumiendo que tiene al menos un nombre
        // ... otras columnas de data_types ...
    }
}

diesel::table! {
    attributes (id) {
        id -> Uuid,
        entity_id -> Uuid, // FK a logical_entities
        data_type_id -> Uuid, // FK a data_types
        name -> Text,
        description -> Nullable<Text>,
        is_required -> Bool, // BOOLEAN -> Bool
        position -> Int2,    // SMALLINT -> Int2
        is_unique -> Nullable<Int2>, // SMALLINT -> Int2
        default_value -> Nullable<Text>,
        validation_regex -> Nullable<Text>,
        created_by -> Nullable<Uuid>, // FK a users
        created_at -> Timestamptz,
        updated_by -> Nullable<Uuid>, // FK a users
        updated_at -> Nullable<Timestamptz>,
        status -> Int2,
    }
}

// --- Definiciones de Joins ---
// Diesel infiere joins simples basados en convenciones o claves foráneas.
// Para joins más complejos o ambiguos (como múltiples FK a la misma tabla),
// a menudo se usan alias *dentro* de las consultas, no definidos aquí.
// Las llamadas a `joinable!` ayudan a Diesel a verificar la validez de los joins.

// Joins para logical_entities (asumiendo FK a users.id)
diesel::joinable!(logical_entities -> users (created_by));
//diesel::joinable!(logical_entities -> users (updated_by)); // Necesitarás alias en la consulta si usas ambos joins a users

// Joins para attributes
diesel::joinable!(attributes -> logical_entities (entity_id));
diesel::joinable!(attributes -> data_types (data_type_id));
diesel::joinable!(attributes -> users (created_by));
//diesel::joinable!(attributes -> users (updated_by)); // Necesitarás alias en la consulta si usas ambos joins a users


// --- Permitir tablas en la misma query ---
// Esto le dice a Diesel que estas tablas pueden aparecer juntas en una consulta.
diesel::allow_tables_to_appear_in_same_query!(
    users,
    logical_entities,
    data_types,
    attributes,
);


// --- Módulos de alias (Opcional pero útil) ---

// Alias original para users
pub mod user {
    pub use super::users::dsl::*;
    pub const table: super::users::table = super::users::table;
}
