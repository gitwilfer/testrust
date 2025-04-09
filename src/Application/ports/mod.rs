pub mod repositories;
pub mod unit_of_work; // Nuevo módulo

pub use repositories::UserRepositoryPort;
pub use repositories::UserQueryRepository; // Nuevas exportaciones
pub use repositories::UserCommandRepository;
pub use repositories::AuthServicePort;
pub use unit_of_work::UnitOfWork;
pub use unit_of_work::RepositoryRegistry;