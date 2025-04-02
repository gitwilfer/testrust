use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateUserDto {
    #[validate(length(min = 3, max = 50))]
    pub first_name: Option<String>,
    #[validate(length(min = 3, max = 50))]
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 6, max = 100))]
    pub password: Option<String>,
}
