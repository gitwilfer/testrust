use std::sync::Arc;
use anyhow::Result;
use log::{debug, trace};
use std::collections::HashMap;
use std::any::{Any, TypeId};
use sqlx::{Pool, Postgres};

use crate::Application::use_cases::user::{
    CreateUserUseCase, FindUserByIdUseCase, FindUserByUsernameUseCase,
    FindAllUsersUseCase, UpdateUserUseCase, DeleteUserUseCase,
    LoginUseCase, CreateUserWithPreferencesUseCase, FindUserByUsernameOptimizedUseCase
};
use crate::Application::ports::repositories::{
    UserRepositoryPort, UserQueryRepository, AuthServicePort
};
use crate::Application::mappers::UserMapper;

/// Estructura que contiene todas las dependencias creadas
#[derive(Clone)]
pub struct AppDependencies {
    // Repositorios y servicios
    pub user_repository: Arc<dyn UserRepositoryPort>,
    pub user_query_repository: Arc<dyn UserQueryRepository>,
    pub auth_service: Arc<dyn AuthServicePort>,
    pub user_mapper: Arc<UserMapper>,
    
    // Casos de uso
    pub create_user_with_preferences_use_case: Arc<CreateUserWithPreferencesUseCase>,
    pub create_user_use_case: Arc<CreateUserUseCase>,
    pub find_user_by_id_use_case: Arc<FindUserByIdUseCase>,
    pub find_all_users_use_case: Arc<FindAllUsersUseCase>,
    pub update_user_use_case: Arc<UpdateUserUseCase>,
    pub delete_user_use_case: Arc<DeleteUserUseCase>,
    pub find_user_by_username_use_case: Arc<dyn crate::Application::use_cases::traits::FindUserByUsernameUseCase>,
    pub login_use_case: Arc<LoginUseCase>,
}

/// Trait que define un proveedor de dependencias
/// Nota: Este trait no se usará como object trait (dyn)
pub trait DependencyProvider {
    /// Obtiene una instancia de un tipo específico
    fn get<T: 'static + Send + Sync + ?Sized>(&self) -> Option<Arc<T>>;
    
    /// Registra una instancia de un tipo específico
    fn register<T: 'static + Send + Sync + ?Sized>(&mut self, instance: Arc<T>);
    
    /// Comprueba si existe una instancia del tipo especificado
    fn has<T: 'static + Send + Sync + ?Sized>(&self) -> bool;
    
    /// Construye AppDependencies con todas las dependencias registradas
    fn build(&self) -> AppDependencies;
}

/// Implementación predeterminada del proveedor de dependencias
pub struct DefaultDependencyProvider {
    // Mapa para almacenar instancias por tipo
    instances: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    
    // Pool de SQLx opcional
    sqlx_pool: Option<Arc<Pool<Postgres>>>,
}

impl DefaultDependencyProvider {
    /// Crea un nuevo proveedor de dependencias
    pub fn new() -> Result<Self> {
        debug!("Creando nuevo DefaultDependencyProvider");
        Ok(Self {
            instances: HashMap::new(),
            sqlx_pool: None,
        })
    }
    
    /// Establece el pool de SQLx para optimización de consultas
    pub fn set_sqlx_pool(&mut self, pool: Arc<Pool<Postgres>>) {
        self.sqlx_pool = Some(pool);
    }
    
    /// Obtiene el pool de SQLx si está disponible
    pub fn get_sqlx_pool(&self) -> Option<Arc<Pool<Postgres>>> {
        self.sqlx_pool.clone()
    }
}

impl DependencyProvider for DefaultDependencyProvider {
    /// Obtiene una instancia registrada por su tipo
    fn get<T: 'static + Send + Sync + ?Sized>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<Arc<T>>();
        trace!("Obteniendo dependencia de tipo: {}", std::any::type_name::<T>());
        
        self.instances.get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<Arc<T>>())
            .map(|arc| arc.clone())
    }
    
    /// Registra una instancia para un tipo específico
    fn register<T: 'static + Send + Sync + ?Sized>(&mut self, instance: Arc<T>) {
        let type_id = TypeId::of::<Arc<T>>();
        trace!("Registrando dependencia de tipo: {}", std::any::type_name::<T>());
        
        self.instances.insert(type_id, Box::new(instance));
    }
    
    /// Comprueba si existe una instancia del tipo especificado
    fn has<T: 'static + Send + Sync + ?Sized>(&self) -> bool {
        let type_id = TypeId::of::<Arc<T>>();
        self.instances.contains_key(&type_id)
    }
    
    /// Construye AppDependencies con todas las dependencias registradas
    fn build(&self) -> AppDependencies {
        debug!("Construyendo AppDependencies desde el proveedor");
        
        // Obtener todas las dependencias necesarias
        let user_repository = self.get::<dyn UserRepositoryPort>()
            .expect("UserRepositoryPort no registrado");
            
        let user_query_repository = self.get::<dyn UserQueryRepository>()
            .expect("UserQueryRepository no registrado");
            
        let auth_service = self.get::<dyn AuthServicePort>()
            .expect("AuthServicePort no registrado");
            
        let user_mapper = self.get::<UserMapper>()
            .expect("UserMapper no registrado");
            
        let create_user_with_preferences_use_case = self.get::<CreateUserWithPreferencesUseCase>()
            .expect("CreateUserWithPreferencesUseCase no registrado");
            
        let create_user_use_case = self.get::<CreateUserUseCase>()
            .expect("CreateUserUseCase no registrado");
            
        let find_user_by_id_use_case = self.get::<FindUserByIdUseCase>()
            .expect("FindUserByIdUseCase no registrado");
            
        let find_all_users_use_case = self.get::<FindAllUsersUseCase>()
            .expect("FindAllUsersUseCase no registrado");
            
        let update_user_use_case = self.get::<UpdateUserUseCase>()
            .expect("UpdateUserUseCase no registrado");
            
        let delete_user_use_case = self.get::<DeleteUserUseCase>()
            .expect("DeleteUserUseCase no registrado");
        
        // Para FindUserByUsernameUseCase, podría ser la implementación estándar o la optimizada
        let find_user_by_username_use_case = if self.has::<FindUserByUsernameOptimizedUseCase>() {
            let use_case = self.get::<FindUserByUsernameOptimizedUseCase>()
                .expect("FindUserByUsernameOptimizedUseCase no registrado");
            use_case as Arc<dyn crate::Application::use_cases::traits::FindUserByUsernameUseCase>
        } else {
            let use_case = self.get::<FindUserByUsernameUseCase>()
                .expect("FindUserByUsernameUseCase no registrado");
            use_case as Arc<dyn crate::Application::use_cases::traits::FindUserByUsernameUseCase>
        };
            
        let login_use_case = self.get::<LoginUseCase>()
            .expect("LoginUseCase no registrado");
        
        // Construir y devolver AppDependencies
        AppDependencies {
            user_repository,
            user_query_repository,
            auth_service,
            user_mapper,
            create_user_with_preferences_use_case,
            create_user_use_case,
            find_user_by_id_use_case,
            find_all_users_use_case,
            update_user_use_case,
            delete_user_use_case,
            find_user_by_username_use_case,
            login_use_case,
        }
    }
}