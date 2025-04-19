pub mod user_command_repository_impl;
pub mod sqlx_repository_base;
pub mod user_query_repository_sqlx;
pub mod sqlx_repository_cached;
pub mod sqlx_batch_repository;

// Declarar módulos para Logical Entity, Attribute, DataType
pub mod logical_entity_command_repository_impl;
pub mod logical_entity_query_repository_impl;
pub mod attribute_command_repository_impl;
pub mod data_type_query_repository_impl;


pub use user_command_repository_impl::UserCommandRepositoryImpl;
pub use user_query_repository_sqlx::UserQueryRepositorySqlx;
pub use sqlx_repository_base::SqlxRepositoryBase;

// Exportar las implementaciones correspondientes
pub use logical_entity_command_repository_impl::LogicalEntityCommandRepositoryImpl;
pub use logical_entity_query_repository_impl::LogicalEntityQueryRepositoryImpl;
pub use attribute_command_repository_impl::AttributeCommandRepositoryImpl;
pub use data_type_query_repository_impl::DataTypeQueryRepositoryImpl;