use actix_web::body::EitherBody;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures::future::{Ready, ok};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct RefreshMiddleware;

impl<S, B> Transform<S, ServiceRequest> for RefreshMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RefreshMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RefreshMiddlewareService { service })
    }
}

pub struct RefreshMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RefreshMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let refresh = req.query_string().contains("refresh=true");
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            if refresh {
                res.headers_mut().insert(
                    HeaderName::from_static("hx-refresh"),
                    HeaderValue::from_static("true"),
                );
            }

            Ok(res.map_into_left_body())
        })
    }
}
