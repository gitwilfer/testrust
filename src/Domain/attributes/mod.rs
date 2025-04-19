// src/Domain/attributes/mod.rs
pub mod attribute;
pub mod repository;
pub mod value_objects;

pub use attribute::Attribute;
pub use repository::{AttributeRepository, AttributeRepositoryResult};
// Exportar value objects específicos si es necesario
pub use value_objects::{AttributeName, AttributeDescription, DefaultValue, ValidationRegex, ValueError};