// Este middleware registra todas las peticiones entrantes y las respuestas salientes.
// Registra el método, la ruta, las cabeceras, la información de conexión, el código de estado y el tiempo transcurrido.
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use log::{info, debug};
use std::future::Future;
//use std::pin::Pin;
use std::time::Instant;

// Estructura para el middleware
pub struct RequestLoggerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for RequestLoggerMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggerMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestLoggerMiddlewareService { service })
    }
}

pub struct RequestLoggerMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddlewareService<S>
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
        let start_time = Instant::now();

        debug!("Request: {} {} - Headers: {:?} - Connection Info: {:?}", method, path, headers, connection_info);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let elapsed_time = start_time.elapsed();
            info!("Response: {} {} - {} - Time: {:?}", method, path, res.status(), elapsed_time);
            Ok(res)
        })
    }
}
