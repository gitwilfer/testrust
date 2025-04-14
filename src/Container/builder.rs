use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use log::{debug, trace};
// use actix_web::web;

// Mapa tipado para almacenar instancias de cualquier tipo que implemente 'Any'
pub struct Registry {
    instances: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            instances: HashMap::new(),
        }
    }

    // Registrar una instancia de un tipo específico
    pub fn register<T: 'static + Send + Sync>(&mut self, instance: T) {
        let type_id = TypeId::of::<T>();
        trace!("Registrando instancia de tipo: {}", std::any::type_name::<T>());
        self.instances.insert(type_id, Box::new(instance));
    }

    // Registrar una instancia ya envuelta en Arc
    pub fn register_arc<T: 'static + Send + Sync>(&mut self, instance: Arc<T>) {
        let type_id = TypeId::of::<Arc<T>>();
        trace!("Registrando Arc de tipo: {}", std::any::type_name::<T>());
        self.instances.insert(type_id, Box::new(instance));
    }

    // Obtener una referencia a una instancia previamente registrada
    pub fn get<T: 'static + Send + Sync>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        trace!("Obteniendo instancia de tipo: {}", std::any::type_name::<T>());
        self.instances
            .get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    // Obtener una instancia Arc previamente registrada
    pub fn get_arc<T: 'static + Send + Sync>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<Arc<T>>();
        trace!("Obteniendo Arc de tipo: {}", std::any::type_name::<T>());
        self.instances
            .get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<Arc<T>>())
            .cloned() // Solo clonamos el Arc, no su contenido
    }
}

// El Builder que construirá nuestro AppState
pub struct ContainerBuilder {
    registry: Registry,
}

impl ContainerBuilder {
    pub fn new() -> Self {
        debug!("Creando nuevo ContainerBuilder");
        ContainerBuilder {
            registry: Registry::new(),
        }
    }

    // Registrar un servicio como un tipo concreto
    pub fn register_service<T>(&mut self, instance: T) -> &mut Self
    where
        T: 'static + Send + Sync,
    {
        debug!("Registrando servicio de tipo: {}", std::any::type_name::<T>());
        self.registry.register(instance);
        self
    }

    // Registrar un servicio ya envuelto en Arc
    pub fn register_arc_service<T>(&mut self, instance: Arc<T>) -> &mut Self
    where
        T: 'static + Send + Sync,
    {
        debug!("Registrando servicio Arc de tipo: {}", std::any::type_name::<T>());
        self.registry.register_arc(instance);
        self
    }

    // Acceder al registro
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    // Construir el AppState final
    pub fn build(self) -> Result<crate::Container::app_state::AppState> {
        debug!("Construyendo AppState a partir del ContainerBuilder");
        Ok(crate::Container::app_state::AppState::new(self.registry))
    }
}

// Función helper para crear un nuevo ContainerBuilder
pub fn create_builder() -> ContainerBuilder {
    ContainerBuilder::new()
}