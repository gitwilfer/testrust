use std::sync::Arc;
use actix_web::web;
use log::{debug, trace, error};

use crate::Container::builder::Registry;
use crate::Presentation::api::controllers::{
    AuthController,
    UserController,
    HealthController,
    LogicalEntityController
};

/// Estado compartido de la aplicación que proporciona acceso a todas las dependencias
#[derive(Clone)]
pub struct AppState {
    registry: Arc<Registry>, // Cambiado &lt; a <
    pub auth_controller_data: web::Data<AuthController>, // Cambiado &lt; a <
    pub user_controller_data: web::Data<UserController>, // Cambiado &lt; a <
    pub health_controller_data: web::Data<HealthController>, // Cambiado &lt; a <
    pub logical_entity_controller_data: web::Data<LogicalEntityController>, // Cambiado &lt; a <
}

impl AppState {
    /// Constructor interno usado por el ContainerBuilder
    pub(crate) fn new(registry: Registry) -> Self {
        debug!("Creando nuevo AppState");

        // Obtener Arc<Controller> directamente
        // Corregir sintaxis ::<> a ::<>
        let auth_controller_arc = registry.get_arc::<AuthController>()
            .expect("AuthController no registrado");

        let user_controller_arc = registry.get_arc::<UserController>()
            .expect("UserController no registrado");

        let health_controller_arc = registry.get_arc::<HealthController>()
            .expect("HealthController no registrado");

        let logical_entity_controller_arc: Arc<LogicalEntityController> = registry.get_arc::<LogicalEntityController>()
            .expect("LogicalEntityController no registrado");

        // Crear web::Data usando los Arc
        let auth_controller_data = web::Data::from(auth_controller_arc);
        let user_controller_data = web::Data::from(user_controller_arc);
        let health_controller_data = web::Data::from(health_controller_arc);
        let logical_entity_controller_data = web::Data::from(logical_entity_controller_arc);

        AppState {
            registry: Arc::new(registry),
            auth_controller_data,
            user_controller_data,
            health_controller_data,
            logical_entity_controller_data,
        }
    }

    /// Obtiene una dependencia del tipo especificado
    pub fn get<T: 'static + Send + Sync>(&self) -> Option<&T> { // Cambiado &lt; a <
        // Corregir sintaxis ::<> a ::<>
        trace!("Obteniendo servicio de tipo: {}", std::any::type_name::<T>());
        self.registry.get::<T>()
    }

    /// Obtiene una dependencia como web::Data para Actix
    pub fn get_web_data<T: 'static + Clone + Send + Sync>(&self) -> Result<web::Data<T>, &'static str> { // Cambiado &lt; a <
        // Corregir sintaxis ::<> a ::<>
        match self.registry.get::<T>() {
            Some(instance) => {
                let cloned = instance.clone();
                // Corregir sintaxis ::<> a ::<>
                trace!("Creando web::Data para tipo: {}", std::any::type_name::<T>());
                Ok(web::Data::new(cloned))
            },
            None => {
                // Corregir sintaxis ::<> a ::<>
                error!("Dependencia no registrada: {}", std::any::type_name::<T>());
                Err("Dependencia no registrada")
            }
        }
    }

    /// Configura una aplicación Actix con todos los controladores necesarios
    pub fn configure_app<F, T>(&self, app_builder: F) -> T // Cambiado &lt; a <
    where
        F: FnOnce(web::Data<AuthController>, // Cambiado &lt; a <
            web::Data<UserController>, // Cambiado &lt; a <
            web::Data<HealthController>, // Cambiado &lt; a <
            web::Data<LogicalEntityController> // Cambiado &lt; a <
        ) -> T,
    {
        debug!("Configurando aplicación Actix con controladores");
        app_builder(
            self.auth_controller_data.clone(),
            self.user_controller_data.clone(),
            self.health_controller_data.clone(),
            self.logical_entity_controller_data.clone()
        )
    }
}
