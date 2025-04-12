use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::{Result, anyhow};
use log::{debug, trace};

/// Registro tipado para almacenar instancias de cualquier tipo
pub struct Registry {
    instances: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Registry {
    /// Crea un nuevo registro vacío
    pub fn new() -> Self {
        Registry {
            instances: HashMap::new(),
        }
    }

    /// Registra una instancia de un tipo específico
    pub fn register<T: 'static + Send + Sync>(&mut self, instance: T) {
        let type_id = TypeId::of::<T>();
        trace!("Registrando instancia de tipo: {}", std::any::type_name::<T>());
        self.instances.insert(type_id, Box::new(instance));
    }

    /// Obtiene una referencia a una instancia previamente registrada
    pub fn get<T: 'static + Send + Sync>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        trace!("Obteniendo instancia de tipo: {}", std::any::type_name::<T>());
        self.instances
            .get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}

/// Constructor de contenedores que simplifica el registro de dependencias
pub struct ContainerBuilder {
    registry: Registry,
}

impl ContainerBuilder {
    /// Crea un nuevo constructor de contenedores
    pub fn new() -> Self {
        debug!("Creando nuevo ContainerBuilder");
        ContainerBuilder {
            registry: Registry::new(),
        }
    }

    /// Registra una implementación de un trait o tipo concreto
    pub fn register_service<T>(&mut self, instance: T) -> &mut Self
    where
        T: 'static + Send + Sync,
    {
        debug!("Registrando servicio de tipo: {}", std::any::type_name::<T>());
        self.registry.register(instance);
        self
    }

    /// Accede al registro interno
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Construye el AppState final con todas las dependencias registradas
    pub fn build(self) -> Result<crate::container::app_state::AppState> {
        debug!("Construyendo AppState a partir del ContainerBuilder");
        Ok(crate::container::app_state::AppState::new(self.registry))
    }
}

/// Función helper para crear un nuevo ContainerBuilder
pub fn create_builder() -> ContainerBuilder {
    ContainerBuilder::new()
}