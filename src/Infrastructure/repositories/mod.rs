pub mod user_repository_impl;
pub mod user_query_repository_impl; // Nuevos archivos
pub mod user_command_repository_impl;
pub mod sqlx_repository_base;
pub mod user_query_repository_sqlx;
pub mod sqlx_repository_cached;
pub mod sqlx_batch_repository;




pub use user_repository_impl::UserRepositoryImpl;
pub use user_query_repository_impl::UserQueryRepositoryImpl;
pub use user_command_repository_impl::UserCommandRepositoryImpl; // Nuevo módulo
pub use user_query_repository_sqlx::UserQueryRepositorySqlx; // Nuevo módulo
pub use sqlx_repository_base::SqlxRepositoryBase; // Nuevo módulo