pub mod user_model;
//pub mod entity;
pub mod logical_entity_model; // Declarar el módulo como público

pub use user_model::UserModel;
pub use user_model::UpdateUserChangeset;
//pub use entity::EntityModel;
pub use logical_entity_model::LogicalEntityModel; // Reexportar el struct
