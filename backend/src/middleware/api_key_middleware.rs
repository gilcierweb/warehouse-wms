use actix_web::{
    Error, HttpResponse,
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::{LocalBoxFuture, Ready, ready};
use std::rc::Rc;

/// Actix-Web middleware that validates the `X-API-Key` header.
/// Blocks all requests without a valid API key.
pub struct ApiKeyAuth {
    api_key: String,
}

impl ApiKeyAuth {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = ApiKeyAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyAuthMiddleware {
            service: Rc::new(service),
            api_key: self.api_key.clone(),
        }))
    }
}

pub struct ApiKeyAuthMiddleware<S> {
    service: Rc<S>,
    api_key: String,
}

impl<S, B> Service<ServiceRequest> for ApiKeyAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let api_key = self.api_key.clone();

        Box::pin(async move {
            // Allow OPTIONS requests (CORS preflight) without API key
            if req.method() == actix_web::http::Method::OPTIONS {
                let res = svc.call(req).await?;
                return Ok(res.map_into_boxed_body());
            }

            // Extract X-API-Key header
            let provided_key = req
                .headers()
                .get("X-API-Key")
                .and_then(|h| h.to_str().ok());

            match provided_key {
                Some(key) if key == api_key => {
                    // API key is valid, continue to handler
                    let res = svc.call(req).await?;
                    Ok(res.map_into_boxed_body())
                }
                _ => {
                    // Missing or invalid API key
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "error": "Unauthorized",
                            "message": "Invalid or missing API key. Provide X-API-Key header."
                        }))
                        .map_into_boxed_body();

                    let (http_req, _payload) = req.into_parts();
                    Ok(ServiceResponse::new(http_req, response))
                }
            }
        })
    }
}
