// src/Infrastructure/Persistence/sqlx_mapper.rs

// use uuid::Uuid;
// use chrono::{NaiveDateTime, DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::{Row, Error};
use anyhow::Result;

use crate::Domain::entities::user::User;
use crate::Domain::entities::entity::Entity;
use crate::Domain::entities::role::Role;

/// Trait para mapear resultados de SQLx a entidades de dominio
pub trait SqlxMapper<T> {
    fn map_row(row: PgRow) -> Result<T, Error>;
}

/// Implementación para User
pub struct UserMapper;

impl SqlxMapper<User> for UserMapper {
    fn map_row(row: PgRow) -> Result<User, Error> {
        let user = User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            email: row.try_get("email")?,
            password: row.try_get("password")?,
            status: row.try_get("status")?,
            created_by: row.try_get("created_by")?,
            created_at: row.try_get("created_at")?,
            modified_by: row.try_get("modified_by")?,
            modified_at: row.try_get("modified_at")?,
        };
        
        Ok(user)
    }
}

/// Implementación para Entity
pub struct EntityMapper;

impl SqlxMapper<Entity> for EntityMapper {
    fn map_row(row: PgRow) -> Result<Entity, Error> {
        let entity = Entity {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            active: row.try_get("active")?,
            created_at: row.try_get("created_at")?,
            created_by: row.try_get("created_by")?,
            modified_at: row.try_get("modified_at")?,
            modified_by: row.try_get("modified_by")?,
            deleted_at: row.try_get("deleted_at")?,
            deleted_by: row.try_get("deleted_by")?,
        };
        
        Ok(entity)
    }
}

/// Implementación para Role
pub struct RoleMapper;

impl SqlxMapper<Role> for RoleMapper {
    fn map_row(row: PgRow) -> Result<Role, Error> {
        let role = Role {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            active: row.try_get("active")?,
            created_at: row.try_get("created_at")?,
            created_by: row.try_get("created_by")?,
            modified_at: row.try_get("modified_at")?,
            modified_by: row.try_get("modified_by")?,
            deleted_at: row.try_get("deleted_at")?,
            deleted_by: row.try_get("deleted_by")?,
        };
        
        Ok(role)
    }
}

// Funciones helper para simplificar el mapeo
pub async fn map_optional_row<T, M>(row_result: Result<Option<PgRow>, Error>) -> Result<Option<T>, Error> 
where 
    M: SqlxMapper<T>
{
    match row_result {
        Ok(Some(row)) => Ok(Some(M::map_row(row)?)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn map_rows<T, M>(rows_result: Result<Vec<PgRow>, Error>) -> Result<Vec<T>, Error> 
where 
    M: SqlxMapper<T>
{
    match rows_result {
        Ok(rows) => {
            let mut entities = Vec::with_capacity(rows.len());
            for row in rows {
                entities.push(M::map_row(row)?);
            }
            Ok(entities)
        },
        Err(e) => Err(e),
    }
}