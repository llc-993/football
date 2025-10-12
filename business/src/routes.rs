// 路由配置
use actix_web::web;
use crate::handle;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        // 基础路由
        .route("/", web::get().to(handle::index))
        .route("/health", web::get().to(handle::health_check))
        
        // API 路由
        .service(
            web::scope("/api")
                // Wallet 钱包路由
                .service(
                    web::scope("/wallet")
                        // 查询钱包余额: GET /api/wallet/{user_id}
                        .route("/{user_id}", web::get().to(handle::get_wallet_balance))

                )
                
                // i18n 测试路由
                .service(
                    web::scope("/test")
                        .route("/error", web::get().to(handle::test_error))
                        .route("/delete", web::get().to(handle::test_delete))
                        .route("/lang", web::get().to(handle::language_demo))
                )
        );
}

