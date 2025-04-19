// src/Application/use_cases/logical_entities/create_logical_entity.rs

use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use thiserror::Error;
use log::{error, info, debug}; // Añadido debug
use anyhow::{Result, anyhow, Context}; // Necesario para UoW y errores
use std::collections::HashSet; // Para validar nombres de atributo duplicados

// --- Importar Ports ---
use crate::Application::ports::unit_of_work::{UnitOfWork, RepositoryRegistry};
use crate::Application::ports::driven::repositories::{
    LogicalEntityCommandRepository, // Para crear la entidad
    AttributeCommandRepository,     // Para crear los atributos
    DataTypeQueryRepository,        // Para buscar ID de tipo de dato
    // Opcional: LogicalEntityQueryRepository si quieres verificar existencia de entidad primero
    // LogicalEntityQueryRepository,
};

// --- 1. Comando de Entrada (Como se definió antes) ---
#[derive(Debug, Clone)]
pub struct AttributeDefinitionCommand {
    pub name: String,
    pub description: Option<String>,
    pub data_type_name: String,
    pub position: i16,
    pub is_required: bool,
    pub is_unique: Option<i16>,
    pub default_value: Option<String>,
    pub validation_regex: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateEntityWithAttributesCommand {
    pub entity_name: String,
    pub attributes: Vec<AttributeDefinitionCommand>,
    pub created_by_user_id: Uuid,
}

// --- 2. Errores Específicos del Caso de Uso (Como se definió antes) ---
#[derive(Error, Debug, PartialEq, Clone)] // Añadido Clone para el mapeo de errores
pub enum CreateEntityError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Entity with name '{0}' already exists.")]
    EntityConflict(String),
    #[error("Attribute '{attribute_name}' for entity '{entity_name}' already exists.")]
    AttributeConflict { entity_name: String, attribute_name: String },
    #[error("Data type '{0}' not found.")]
    DataTypeNotFound(String),
    #[error("Database error during operation: {0}")]
    DatabaseError(String),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

// --- 3. Trait del Caso de Uso (Como se definió antes) ---
#[async_trait]
pub trait CreateEntityWithAttributesUseCase: Send + Sync {
    async fn execute(&self, command: CreateEntityWithAttributesCommand) -> Result<Uuid, CreateEntityError>;
}

// --- 4. Implementación del Caso de Uso (COMPLETADA) ---
pub struct CreateEntityWithAttributesUseCaseImpl {
    uow: Arc<dyn UnitOfWork>,
}

impl CreateEntityWithAttributesUseCaseImpl {
    pub fn new(uow: Arc<dyn UnitOfWork>) -> Self {
        Self { uow }
    }
}

#[async_trait]
impl CreateEntityWithAttributesUseCase for CreateEntityWithAttributesUseCaseImpl {
    async fn execute(&self, command: CreateEntityWithAttributesCommand) -> Result<Uuid, CreateEntityError> {
        info!("Executing CreateEntityWithAttributesUseCase for entity: {}", command.entity_name);

        // --- Validación de Negocio Inicial ---
        // 1. Verificar nombres de atributo duplicados en el request
        let mut attribute_names = HashSet::new();
        for attr in &command.attributes {
            if !attribute_names.insert(attr.name.clone()) {
                let error_msg = format!("Duplicate attribute name '{}' provided in the request for entity '{}'", attr.name, command.entity_name);
                error!("{}", error_msg);
                return Err(CreateEntityError::ValidationError(error_msg));
            }
        }
        // 2. Otras validaciones si son necesarias (ej: posición única, etc.)

        // Clonar datos necesarios para la clausura 'async move'
        let entity_name_clone = command.entity_name.clone();
        let attributes_clone = command.attributes.clone();
        let user_id_clone = command.created_by_user_id;

        // --- Ejecutar dentro de la Unidad de Trabajo ---
        let result = self.uow.execute(move |registry: &dyn RepositoryRegistry| async move {
            debug!("Inside Unit of Work for creating entity '{}'", entity_name_clone);

            // Obtener repositorios necesarios DESDE el registry
            let entity_cmd_repo = registry.logical_entity_command_repository();
            let attribute_cmd_repo = registry.attribute_command_repository();
            let data_type_query_repo = registry.data_type_query_repository();
            // Opcional: let entity_query_repo = registry.logical_entity_query_repository();

            // --- (Opcional) Verificar si la entidad ya existe ---
            // Descomentar si se añade LogicalEntityQueryRepository a la UoW y se implementa
            /*
            match entity_query_repo.exists_by_name(&entity_name_clone).await {
                Ok(true) => {
                    let err = CreateEntityError::EntityConflict(entity_name_clone.clone());
                    error!("{}", err);
                    return Err(anyhow!(err));
                }
                Ok(false) => { /* Continuar */ }
                Err(e) => {
                    let err = CreateEntityError::DatabaseError(format!("Failed to check entity existence: {}", e));
                    error!("{}", err);
                    return Err(anyhow!(err));
                }
            }
            */

            // --- Crear la Entidad ---
            debug!("Attempting to create entity '{}'", entity_name_clone);
            let new_entity_id = match entity_cmd_repo.create(
                // La conexión se pasa aquí si el trait/impl lo requiere
                // conn, // <--- Descomentar y pasar si es necesario
                &entity_name_clone,
                None, // Descripción no viene del comando principal, podría añadirse
                None, // Assign view tampoco
                user_id_clone,
            ).await {
                Ok(id) => {
                    info!("Entity '{}' created with ID: {}", entity_name_clone, id);
                    id
                },
                Err(e) => {
                    // Podríamos intentar detectar si el error es por conflicto de nombre
                    // basado en el mensaje de error de la BD (frágil) o código de error.
                    // Por ahora, lo tratamos como error genérico de BD.
                    let err = CreateEntityError::DatabaseError(format!("Failed to create entity: {}", e));
                    error!("{}", err);
                    return Err(anyhow!(err));
                }
            };

            // --- Crear los Atributos (Iterar) ---
            for attr_cmd in attributes_clone {
                debug!("Processing attribute '{}' for entity '{}'", attr_cmd.name, entity_name_clone);

                // 1. Buscar ID del Tipo de Dato por nombre
                let data_type_id = match data_type_query_repo.find_id_by_name(&attr_cmd.data_type_name).await {
                    Ok(Some(id)) => id,
                    Ok(None) => {
                        let err = CreateEntityError::DataTypeNotFound(attr_cmd.data_type_name.clone());
                        error!("Data type lookup failed: {}", err);
                        return Err(anyhow!(err));
                    }
                    Err(e) => {
                        let err = CreateEntityError::DatabaseError(format!("Failed to query data type '{}': {}", attr_cmd.data_type_name, e));
                        error!("{}", err);
                        return Err(anyhow!(err));
                    }
                };
                debug!("Found data_type_id: {} for name '{}'", data_type_id, attr_cmd.data_type_name);

                // 2. Crear el Atributo
                match attribute_cmd_repo.create(
                    // conn, // <--- Descomentar y pasar si es necesario
                    new_entity_id,
                    data_type_id,
                    &attr_cmd.name,
                    attr_cmd.description.as_deref(),
                    attr_cmd.is_required,
                    attr_cmd.position,
                    attr_cmd.is_unique,
                    attr_cmd.default_value.as_deref(),
                    attr_cmd.validation_regex.as_deref(),
                    user_id_clone,
                ).await {
                    Ok(attr_id) => {
                        info!("Attribute '{}' created with ID: {} for entity '{}'", attr_cmd.name, attr_id, entity_name_clone);
                    }
                    Err(e) => {
                        // Intentar detectar conflicto UNIQUE(entity_id, name)
                        // Esto depende del error específico devuelto por Diesel/BD
                        // if e.to_string().contains("violates unique constraint") { ... }
                        let err = CreateEntityError::DatabaseError(format!("Failed to create attribute '{}': {}", attr_cmd.name, e));
                        // Podríamos mapear a AttributeConflict si detectamos el error específico
                        // let err = CreateEntityError::AttributeConflict { entity_name: entity_name_clone.clone(), attribute_name: attr_cmd.name.clone() };
                        error!("{}", err);
                        return Err(anyhow!(err));
                    }
                }
            }

            // Si todo fue bien, retornar el ID de la entidad creada
            Ok(new_entity_id)

        }).await; // Espera a que la UoW termine (commit/rollback)

        // Mapear el resultado de la UoW (Result<Uuid, anyhow::Error>)
        // al tipo de retorno del caso de uso (Result<Uuid, CreateEntityError>)
        result.map_err(|e| {
            // Intenta convertir el error de anyhow de nuevo a CreateEntityError
            match e.downcast::<CreateEntityError>() {
                Ok(app_err) => app_err, // Si ya era nuestro error, lo devolvemos
                Err(other_err) => {
                     // Si no, es un error inesperado de la UoW, BD o conversión
                    error!("Unexpected error during UoW execution: {:?}", other_err);
                    CreateEntityError::Unexpected(other_err.to_string())
                }
            }
        })
    }
}

// --- Tests Unitarios (Necesitarán adaptarse a la nueva lógica y mocks) ---
#[cfg(test)]
mod tests {
    // ... tests necesitarán refactorización completa para mockear UoW, Registry y los 3 repositorios ...
}
