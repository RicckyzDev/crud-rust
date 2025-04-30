use actix_service::{Service, Transform};
use actix_web::body::{BoxBody, EitherBody};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct ErrorHandlerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ErrorHandlerMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ErrorHandlerMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ErrorHandlerMiddlewareService {
            service: Arc::new(service),
        })
    }
}

pub struct ErrorHandlerMiddlewareService<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for ErrorHandlerMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Arc::clone(&self.service);

        Box::pin(async move {
            match service.call(req).await {
                Ok(res) => Ok(res.map_into_left_body()),
                Err(err) => {
                    let error_response = HttpResponse::InternalServerError()
                        .json(serde_json::json!({ "error": format!("{}", err) }))
                        .map_into_right_body();

                    let dummy_req = actix_web::test::TestRequest::default().to_http_request();
                    let dummy_service_req =
                        ServiceRequest::from_parts(dummy_req, actix_web::dev::Payload::None);
                    let (http_req, _) = dummy_service_req.into_parts();

                    Ok(ServiceResponse::new(http_req, error_response))
                }
            }
        })
    }
}
