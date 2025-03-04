use actix_session::Session;
use actix_web::{
    Error, FromRequest,
    body::{BoxBody, EitherBody},
    dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    http::{
        StatusCode,
        header::{self, HeaderValue},
    },
};
use futures::future::LocalBoxFuture;
use std::future::{Ready, ready};

pub struct CheckLogin;

impl<S, B> Transform<S, ServiceRequest> for CheckLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware { service }))
    }
}

pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_owned();
        let fut = self.service.call(req);
        Box::pin(async move {
            if path.starts_with("/static/")
                || path.starts_with("/auth/")
                || path.starts_with("/tests/")
                || path.starts_with("/login")
            {
                let res = fut.await?;
                Ok(res.map_body(|_head, body| EitherBody::left(body)))
            } else {
                let res: ServiceResponse<B> = fut.await?;
                let request = res.request();
                let mut payload: Payload = Payload::None;
                let session: Session = Session::from_request(request, &mut payload).await.unwrap();
                let session_json = session.get::<serde_json::Value>("session").unwrap();
                if session_json.is_some() {
                    return Ok(res.map_body(|head, body| {
                        head.status = StatusCode::FOUND;
                        head.headers_mut()
                            .append(header::LOCATION, HeaderValue::from_static("/login"));
                        EitherBody::left(body)
                    }));
                }
                Ok(res.map_body(|_head, body| EitherBody::left(body)))
            }
        })
    }
}
