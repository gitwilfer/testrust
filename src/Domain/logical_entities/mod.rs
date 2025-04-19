// src/Domain/logical_entities/mod.rs

pub mod logical_entity;
pub mod repository;
pub mod value_objects;

// Re-exportar para facilitar el acceso desde fuera del módulo
pub use logical_entity::LogicalEntity;
pub use repository::LogicalEntityRepository;
// pub use value_objects::*; // Descomentar cuando se añadan VOs