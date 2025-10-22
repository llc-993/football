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
                // User 用户路由
                .service(
                    web::scope("/user")
                        // 注册
                        .route("/register", web::post().to(handle::register))
                        // 登陆
                        .route("/login", web::post().to(handle::login))
                        // 修改密码
                        .route("/change-password", web::post().to(handle::change_password))
                        // 找回密码
                        .route("/reset-password", web::post().to(handle::reset_password))
                        // 详细信息
                        .route("/info", web::get().to(handle::get_user_detail))
                )
                
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

