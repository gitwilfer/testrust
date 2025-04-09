pub mod models;
pub mod database;
pub mod mapper;
pub mod transaction;
pub mod schema;
pub mod unit_of_work; // Nuevo módulo

pub use mapper::user_to_model;
pub use mapper::model_to_user;
pub use transaction::execute_transaction;
pub use unit_of_work::DatabaseUnitOfWork;