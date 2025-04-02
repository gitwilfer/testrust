use actix_web::{error::Error, http::StatusCode, HttpResponse, ResponseError, web};
use serde::Serialize;
use std::fmt;
use validator::Validate;

// Mantenemos las estructuras existentes
#[derive(Debug, Serialize, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ValidationErrors {
    pub errors: Vec<ValidationError>,
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Errores de validación: {:?}", self.errors)
    }
}

impl ResponseError for ValidationErrors {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(self)
    }
}

// Implementamos Send y Sync para ValidationErrors
unsafe impl Send for ValidationErrors {}
unsafe impl Sync for ValidationErrors {}

// Mantenemos la función existente
pub fn validate_request<T>(value: &T) -> Result<(), Error>
where
    T: Validate + ?Sized,
{
    match value.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let validation_errors = ValidationErrors {
                errors: errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, field_errors)| {
                        let field_str = field.to_string(); // Convertimos field a String
                        field_errors.iter().map(move |error| ValidationError {
                            field: field_str.clone(), // Clonamos la String
                            message: error
                                .message
                                .as_ref()
                                .map(|msg| msg.to_string())
                                .unwrap_or_else(|| format!("Campo '{}' inválido", field_str)),
                        })
                    })
                    .collect(),
            };

            Err(Error::from(validation_errors))
        }
    }
}

// Añadimos una nueva función para validar Json
pub fn validate_json<T>(json: &web::Json<T>) -> Result<(), Error>
where
    T: Validate,
{
    validate_request(&json.0)
}
