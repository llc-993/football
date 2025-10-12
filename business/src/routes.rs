// 路由配置
use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        // 基础路由
        .route("/", web::get().to(handlers::index))
        .route("/health", web::get().to(handlers::health_check))
        
        // API 路由
        .service(
            web::scope("/api")
                // i18n 测试路由
                .service(
                    web::scope("/test")
                        .route("/error", web::get().to(handlers::test_error))
                        .route("/delete", web::get().to(handlers::test_delete))
                        .route("/lang", web::get().to(handlers::language_demo))
                )
        );
}

