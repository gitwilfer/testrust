use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::sync::Arc;
use log::debug;

use crate::Application::errors::application_error::ApplicationError;
use crate::Application::ports::driven::AuthServicePort;
use crate::Presentation::api::adapters::ErrorAdapter;
use crate::Infrastructure::auth::auth_service_impl::AuthServiceImpl;

// Middleware para autenticación JWT
#[derive(Clone)]
pub struct AuthMiddleware {
    auth_service: Arc<dyn AuthServicePort>,
}

impl AuthMiddleware {
    pub fn new() -> Self {
        let auth_service_impl = AuthServiceImpl::new()
            .expect("No se pudo crear AuthServiceImpl");
        AuthMiddleware {
            auth_service: Arc::new(auth_service_impl),
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
        if req.path() == "/api/auth/login" {
            debug!("Ruta /api/auth/login excluida de la autenticación.");
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        let auth_service = self.auth_service.clone();
        let mut authenticated = false;
        let mut user_id = None;

        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str[7..].to_string();
                    let token_clone = token.clone();

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

        Box::pin(async move {
            let mut res = fut.await?;

            if let Some(id) = user_id {
                res.request().extensions_mut().insert(id);
            }

            Ok(res)
        })
    }
}