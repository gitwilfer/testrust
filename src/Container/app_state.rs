use std::sync::Arc;
use actix_web::web;
use log::{debug, trace, error};

use crate::Container::builder::Registry;
use crate::Presentation::api::controllers::{AuthController, UserController, HealthController};

/// Estado compartido de la aplicación que proporciona acceso a todas las dependencias
#[derive(Clone)]
pub struct AppState {
    registry: Arc<Registry>,
    
    // Campos para compatibilidad con código existente
    pub auth_controller_data: web::Data<AuthController>,
    pub user_controller_data: web::Data<UserController>,
    pub health_controller_data: web::Data<HealthController>,
}

impl AppState {
    /// Constructor interno usado por el ContainerBuilder
    /// Constructor interno usado por el ContainerBuilder
/// Constructor interno usado por el ContainerBuilder
pub(crate) fn new(registry: Registry) -> Self {
    debug!("Creando nuevo AppState");
    
    // Obtener Arc<Controller> directamente
    let auth_controller = registry.get_arc::<AuthController>()
        .expect("AuthController no registrado");
        
    let user_controller = registry.get_arc::<UserController>()
        .expect("UserController no registrado");
        
    let health_controller = registry.get_arc::<HealthController>()
        .expect("HealthController no registrado");
    
    // Crear web::Data usando los Arc (Actix puede usar Arc internamente)
    let auth_controller_data = web::Data::from(auth_controller);
    let user_controller_data = web::Data::from(user_controller);
    let health_controller_data = web::Data::from(health_controller);
    
    AppState {
        registry: Arc::new(registry),
        auth_controller_data,
        user_controller_data,
        health_controller_data,
    }
}
        
    /// Obtiene una dependencia del tipo especificado
    pub fn get<T: 'static + Send + Sync>(&self) -> Option<&T> {
        trace!("Obteniendo servicio de tipo: {}", std::any::type_name::<T>());
        self.registry.get::<T>()
    }
    
    /// Obtiene una dependencia como web::Data para Actix
    pub fn get_web_data<T: 'static + Clone + Send + Sync>(&self) -> Result<web::Data<T>, &'static str> {
        match self.registry.get::<T>() {
            Some(instance) => {
                let cloned = instance.clone();
                trace!("Creando web::Data para tipo: {}", std::any::type_name::<T>());
                Ok(web::Data::new(cloned))
            },
            None => {
                error!("Dependencia no registrada: {}", std::any::type_name::<T>());
                Err("Dependencia no registrada")
            }
        }
    }
    
    /// Configura una aplicación Actix con todos los controladores necesarios
    pub fn configure_app<F, T>(&self, app_builder: F) -> T 
    where
        F: FnOnce(web::Data<AuthController>, web::Data<UserController>, web::Data<HealthController>) -> T,
    {
        debug!("Configurando aplicación Actix con controladores");
        app_builder(
            self.auth_controller_data.clone(),
            self.user_controller_data.clone(),
            self.health_controller_data.clone()
        )
    }
}