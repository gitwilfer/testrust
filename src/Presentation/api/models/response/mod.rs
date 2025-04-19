mod user_response;
mod token_response;
pub mod logical_entity_response;

pub use user_response::UserResponse;
pub use token_response::TokenResponse;
pub use logical_entity_response::{LogicalEntityResponse, CreateLogicalEntityResponse}; // <--- AÑADIR (elige una o ambas según necesites)
