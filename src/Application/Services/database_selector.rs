use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::Infrastructure::Persistence::connection_pools::{
    get_default_sqlx_db_name_sync, // Cambiado a la versión síncrona
    get_sqlx_pool,
};
use log::debug;

// Servicio para seleccionar la base de datos según la lógica de negocio
pub struct DatabaseSelector {
    // Mapa de entidades a bases de datos
    entity_mappings: HashMap<String, String>,
    // Mapa de tenants a bases de datos (para arquitecturas multi-tenant)
    tenant_mappings: HashMap<Uuid, String>,
    // Base de datos predeterminada
    default_db: String,
}

impl DatabaseSelector {
    pub fn new(default_db: &str) -> Self {
        Self {
            entity_mappings: HashMap::new(),
            tenant_mappings: HashMap::new(),
            default_db: default_db.to_string(),
        }
    }
    
    // Registrar una entidad para usar una base de datos específica
    pub fn register_entity(&mut self, entity_name: &str, database: &str) {
        self.entity_mappings.insert(entity_name.to_string(), database.to_string());
    }
    
    // Registrar un tenant para usar una base de datos específica
    pub fn register_tenant(&mut self, tenant_id: Uuid, database: &str) {
        self.tenant_mappings.insert(tenant_id, database.to_string());
    }
    
    // Obtener la base de datos para una entidad
    pub fn get_database_for_entity(&self, entity_name: &str) -> String {
        self.entity_mappings.get(entity_name)
            .cloned()
            .unwrap_or_else(|| self.default_db.clone())
    }
    
    // Obtener la base de datos para un tenant
    pub fn get_database_for_tenant(&self, tenant_id: Uuid) -> String {
        self.tenant_mappings.get(&tenant_id)
            .cloned()
            .unwrap_or_else(|| self.default_db.clone())
    }
    
    // Obtener la base de datos predeterminada
    pub fn get_default_database(&self) -> String {
        self.default_db.clone()
    }
}

// Singleton para acceso global al selector de bases de datos
lazy_static! {
    static ref DB_SELECTOR: Arc<Mutex<DatabaseSelector>> = {
        // Usar la función síncrona y manejar el Option
        let default_db_name = get_default_sqlx_db_name_sync()
            .unwrap_or_else(|| {
                // Si no hay default configurado (o aún no está listo), usar "main"
                log::warn!("No se encontró nombre de DB SQLx por defecto en get_default_sqlx_db_name_sync. Usando 'main'.");
                "main".to_string()
            });
        Arc::new(Mutex::new(
            DatabaseSelector::new(&default_db_name)
        ))
    };
}

// Inicializar mapeado de entidades a bases de datos
pub fn initialize_database_mappings() {
    let mut selector = DB_SELECTOR.lock().unwrap();
    
    // Ejemplo: Configurar entidades específicas para usar bases de datos particulares
    // Esto podría cargarse desde configuración
    selector.register_entity("user", "main");
    selector.register_entity("audit_log", "analytics");
    
    // También puedes configurar tenants si tu aplicación es multi-tenant
    // selector.register_tenant(Uuid::parse_str("...").unwrap(), "tenant_1_db");
}

// Funciones de acceso público para el selector
pub fn get_database_for_entity(entity_name: &str) -> String {
    let selector = DB_SELECTOR.lock().unwrap();
    selector.get_database_for_entity(entity_name)
}

pub fn get_database_for_tenant(tenant_id: Uuid) -> String {
    let selector = DB_SELECTOR.lock().unwrap();
    selector.get_database_for_tenant(tenant_id)
}

pub fn get_default_database() -> String {
    let selector = DB_SELECTOR.lock().unwrap();
    selector.get_default_database()
}