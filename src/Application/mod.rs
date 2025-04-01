// # Capa de Aplicación
// 
// Esta capa contiene la lógica de aplicación y coordina el flujo de datos entre
// la capa de presentación y la capa de dominio.
// 
// ## Integración con la capa de presentación
// 
// La capa de aplicación se integra con la capa de presentación a través de:
// 
// - **DTOs**: Los Data Transfer Objects definen la estructura de los datos que se
//   intercambian entre las capas.
// - **Casos de uso**: Implementan la lógica de negocio y son invocados por los
//   controladores de la capa de presentación.
// - **Manejo de errores**: Los errores de la capa de aplicación se mapean a
//   respuestas HTTP en la capa de presentación.
// 
// ## Convenciones
// 
// - Los casos de uso deben devolver `Result<T, ApplicationError>` para un manejo
//   consistente de errores.
// - Los DTOs deben implementar `Validate` para validación y tener métodos de
//   conversión desde/hacia los modelos de la capa de presentación.
// - Las transacciones deben iniciarse en la capa de aplicación, no en la capa
//   de presentación.

pub mod dtos;
pub mod use_cases;
pub mod mappers;
pub mod validators;
pub mod errors;
pub mod ports;
