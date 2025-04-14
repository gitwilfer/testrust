// Este middleware maneja los errores que ocurren durante el procesamiento de una petición.
// Registra el error y devuelve una respuesta HTTP apropiada al cliente.
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error
    // Error, HttpResponse, http::StatusCode, error::ResponseError
};
use futures::future::{ok, LocalBoxFuture, Ready};
use log::error;
use crate::Presentation::api::middleware::map_error_thread_safe;


// Estructura para el middleware
pub struct ErrorHandlerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ErrorHandlerMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ErrorHandlerMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ErrorHandlerMiddlewareService { service })
    }
}

pub struct ErrorHandlerMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ErrorHandlerMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.path().to_owned();
        let headers = req.headers().clone();
        let connection_info = req.connection_info().clone();
        let fut = self.service.call(req);

        Box::pin(async move {
            match fut.await {
                Ok(res) => Ok(res),
                Err(err) => {
                    error!("Error en la solicitud: {} {} - Headers: {:?} - Connection Info: {:?} - Error: {}", method, path, headers, connection_info, err);
                    //Err(map_error(err.into()))
                    Err(map_error_thread_safe(err))
                }
            }
        })
    }
}
