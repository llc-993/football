// 中间件：自动解析语言并注入到请求扩展中
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};
use super::i18n::Language;

/// 语言检测中间件
pub struct LanguageMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LanguageMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LanguageMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LanguageMiddlewareService { service }))
    }
}

pub struct LanguageMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LanguageMiddlewareService<S>
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
        // 从请求头解析语言
        let lang = req
            .headers()
            .get("Accept-Language")
            .and_then(|h| h.to_str().ok())
            .map(Language::from_accept_language)
            .unwrap_or(Language::ZhCN);

        // 将语言注入到请求扩展中
        req.extensions_mut().insert(lang);

        // 设置当前线程的语言（用于全局翻译）
        super::i18n::set_current_language(lang);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

