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
                .service(
                    web::scope("/users")
                        .route("", web::get().to(handlers::get_users))
                        .route("/{id}", web::get().to(handlers::get_user))
                        .route("/username/{username}", web::get().to(handlers::get_user_by_username))
                )
        );
}

