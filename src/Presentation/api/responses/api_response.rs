use serde::{Deserialize, Serialize};
use crate::presentation::api::responses::api_error::ApiError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: Option<T>, _message: Option<&str>) -> Self {
        ApiResponse {
            success: true,
            data,
            error: None,
        }
    }

    pub fn error(error: ApiError) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}
