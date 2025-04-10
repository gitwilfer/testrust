use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use log::debug;

use crate::Application::errors::application_error::ApplicationError;
use crate::Application::ports::repositories::AuthServicePort;
use crate::presentation::api::adapters::ErrorAdapter;

// Middleware para autenticación JWT
pub struct AuthMiddleware {
    auth_service: Arc<dyn AuthServicePort>,
}

impl AuthMiddleware {
    pub fn new(auth_service: Arc<dyn AuthServicePort>) -> Self {
        AuthMiddleware {
            auth_service,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service,
            auth_service: self.auth_service.clone(),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    auth_service: Arc<dyn AuthServicePort>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<ServiceResponse<B>, Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_service = self.auth_service.clone();
        let mut authenticated = false;
        let mut user_id = None;

        // Extraer token del header Authorization
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str[7..].to_string(); // Eliminar "Bearer "
                    
                    // Clonar el token para evitar problemas de lifetime
                    let token_clone = token.clone();
                    
                    // Validar token (ejecutar de forma sincrónica para simplificar)
                    // En un entorno de producción, esto debería manejarse de forma asíncrona
                    let validation_result = tokio::task::block_in_place(move || {
                        tokio::runtime::Handle::current().block_on(async {
                            auth_service.validate_token(&token_clone).await
                        })
                    });
                    
                    match validation_result {
                        Ok(id) => {
                            authenticated = true;
                            user_id = Some(id);
                            debug!("Usuario autenticado: {:?}", id);
                        },
                        Err(e) => {
                            debug!("Error al validar token: {:?}", e);
                        }
                    }
                }
            }
        }

        let fut = self.service.call(req);

        // Si la autenticación falló, devolver error
        if !authenticated {
            let error = ApplicationError::AuthenticationError("Token inválido o ausente".to_string());
            let http_error = ErrorAdapter::map_application_error(error);
            return Box::pin(async move {
                Err(actix_web::error::InternalError::from_response(
                    "Unauthorized",
                    http_error
                ).into())
            });
        }

        // Si la autenticación tuvo éxito, insertar el ID del usuario en las extensiones
        // para que los controladores puedan acceder a él
        Box::pin(async move {
            let mut res = fut.await?;
            
            if let Some(id) = user_id {
                res.request().extensions_mut().insert(id);
            }
            
            Ok(res)
        })
    }
}