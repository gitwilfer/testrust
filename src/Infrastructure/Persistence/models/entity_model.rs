// src/infrastructure/persistence/models/entity_model.rs
use chrono::NaiveDateTime;
use uuid::Uuid;
use diesel::prelude::*;
use crate::Infrastructure::Persistence::schema::entidades;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = entidades)]
pub struct EntityModel {
    #[diesel(column_name = idx_entidad)]
    pub id: Uuid,
    
    #[diesel(column_name = nombre)]
    pub name: String,
    
    #[diesel(column_name = descripcion)]
    pub description: Option<String>,
    
    #[diesel(column_name = activo)]
    pub active: bool,
    
    #[diesel(column_name = fecha_creacion)]
    pub created_at: NaiveDateTime,
    
    #[diesel(column_name = creado_por)]
    pub created_by: Option<Uuid>,
    
    #[diesel(column_name = fecha_modificacion)]
    pub modified_at: Option<NaiveDateTime>,
    
    #[diesel(column_name = modificado_por)]
    pub modified_by: Option<Uuid>,
}

// Mapper para convertir entre modelos de persistencia y entidades de dominio
pub mod mapper {
    use super::*;
    use crate::Domain::entities::entity::Entity;
    
    /// Convierte una entidad de dominio a un modelo de persistencia
    pub fn entity_to_model(entity: &Entity) -> EntityModel {
        EntityModel {
            id: entity.id,
            name: entity.name.clone(),
            description: entity.description.clone(),
            active: entity.active,
            created_at: entity.created_at,
            created_by: entity.created_by,
            modified_at: entity.modified_at,
            modified_by: entity.modified_by,
        }
    }
    
    /// Convierte un modelo de persistencia a una entidad de dominio
    pub fn model_to_entity(model: &EntityModel) -> Entity {
        Entity {
            id: model.id,
            name: model.name.clone(),
            description: model.description.clone(),
            active: model.active,
            created_at: model.created_at,
            created_by: model.created_by,
            modified_at: model.modified_at,
            modified_by: model.modified_by,
        }
    }
}