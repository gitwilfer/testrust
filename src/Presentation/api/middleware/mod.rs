// Este módulo contiene los middlewares para la API REST.
// Middlewares are used to intercept requests and responses,
// and perform actions such as logging, error handling, etc.

pub mod error_handler;
pub mod request_logger;
pub mod error_mapper;

pub use error_mapper::map_error;
pub use error_mapper::map_error_thread_safe;
//pub use error_mapper::CustomError;
