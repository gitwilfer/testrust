// --- User Repositories ---
pub mod users_repositories;
// --- AJUSTADO: Exportar solo los traits CQRS ---
pub use users_repositories::{UserCommandRepository, UserQueryRepository};

// --- Logical Entity Repositories ---
pub mod logical_entity_command_repository;
pub mod logical_entity_query_repository;
pub use logical_entity_command_repository::LogicalEntityCommandRepository;
pub use logical_entity_query_repository::{LogicalEntityQueryRepository, LogicalEntityDto};

// --- Attribute Repository ---
pub mod attribute_command_repository;
pub use attribute_command_repository::AttributeCommandRepository;

// --- DataType Repository ---
pub mod data_type_query_repository;
pub use data_type_query_repository::DataTypeQueryRepository;
