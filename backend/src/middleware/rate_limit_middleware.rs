#![allow(dead_code)]

use actix_web::{
    Error, HttpResponse,
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::{LocalBoxFuture, Ready, ready};
use serde_json::json;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct RateLimit {
    pub max_requests: u64,
    pub window_secs: u64,
    pub key_prefix: &'static str,
}

impl RateLimit {
    pub const fn new(max_requests: u64, window_secs: u64, key_prefix: &'static str) -> Self {
        Self {
            max_requests,
            window_secs,
            key_prefix,
        }
    }
}

pub const RATE_AUTH: RateLimit = RateLimit::new(1000, 60, "rl:auth");
pub const RATE_API: RateLimit = RateLimit::new(500, 60, "rl:api");
pub const RATE_UPLOAD: RateLimit = RateLimit::new(10, 3600, "rl:upload");
pub const RATE_MESSAGES: RateLimit = RateLimit::new(200, 60, "rl:msg");

pub struct RateLimiter {
    redis: deadpool_redis::Pool,
    limit: RateLimit,
}

impl RateLimiter {
    pub fn new(redis: deadpool_redis::Pool, limit: RateLimit) -> Self {
        Self { redis, limit }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = RateLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service: Rc::new(service),
            redis: self.redis.clone(),
            limit: self.limit.clone(),
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: Rc<S>,
    redis: deadpool_redis::Pool,
    limit: RateLimit,
}

impl<S> RateLimiterMiddleware<S> {
    fn new(service: Rc<S>, redis: deadpool_redis::Pool, limit: RateLimit) -> Self {
        Self {
            service,
            redis,
            limit,
        }
    }
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
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
        let redis = self.redis.clone();
        let limit = self.limit.clone();

        let client_key = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        Box::pin(async move {
            let key = format!("{}:{}", limit.key_prefix, client_key);

            let allowed = async {
                let mut conn = redis.get().await.ok()?;

                let count: u64 = redis::pipe()
                    .atomic()
                    .cmd("INCR")
                    .arg(&key)
                    .cmd("EXPIRE")
                    .arg(&key)
                    .arg(limit.window_secs)
                    .ignore()
                    .query_async::<(u64,)>(&mut conn)
                    .await
                    .ok()?
                    .0;

                Some(count <= limit.max_requests)
            }
            .await
            .unwrap_or(true);

            if !allowed {
                let response = HttpResponse::TooManyRequests()
                    .insert_header(("Retry-After", limit.window_secs.to_string()))
                    .insert_header(("X-RateLimit-Limit", limit.max_requests.to_string()))
                    .json(json!({
                        "error": {
                            "code": "RATE_LIMITED",
                            "message": t!("errors.rate_limited")
                        }
                    }))
                    .map_into_boxed_body();

                let (http_req, _payload) = req.into_parts();
                return Ok(ServiceResponse::new(http_req, response));
            }

            let res = svc.call(req).await?;
            Ok(res.map_into_boxed_body())
        })
    }
}
