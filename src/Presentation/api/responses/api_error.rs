use actix_web::http::StatusCode;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(skip)]
    status_code: StatusCode,
    code: u16,  // Para serialización/deserialización
    message: String,
}

impl ApiError {
    pub fn new(status_code: StatusCode, message: &str) -> Self {
        ApiError {
            status_code,
            code: status_code.as_u16(),
            message: message.to_string(),
        }
    }
    
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }
    
    pub fn not_found(message: &str) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }
    
    pub fn bad_request(message: &str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }
    
    pub fn internal_server_error(message: &str) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }
}
