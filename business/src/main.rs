use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use football_common::config::AppConfig;
use football_orm::{db, cache};

mod handle;
mod routes;
mod conf;

use conf::{I18nBundles, LanguageMiddleware};
use conf::i18n;

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    // 加载配置（优先从配置文件加载，回退到环境变量）
    let config = AppConfig::from_file("business/config")
        .or_else(|_| AppConfig::from_env())
        .expect("配置加载失败");
    // log::info!("Business 配置加载成功: {:?}", config);

    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(config.log.level));

    // 初始化数据库连接（使用连接池）
    let db_config = football_common::conf::DbConfig::new(
        config.database.url.clone(),
        config.database.max_connections as u64,
    );
    db::init_db(&db_config)
        .await
        .expect("数据库连接池初始化失败");

    // 测试数据库连接
    if let Err(e) = db::test_connection().await {
        log::error!("数据库连接测试失败: {}", e);
    }

    // 初始化 Redis 连接
    let redis_config = football_common::conf::RedisConfig::from_url(
        config.redis.url.clone(),
        config.redis.pool_size,
    );
    cache::init_redis(&redis_config).expect("Redis初始化失败");

    // 测试 Redis 连接
    if let Err(e) = cache::test_connection().await {
        log::error!("Redis连接测试失败: {}", e);
    }

    // 初始化 Fluent 多语言资源
    let i18n = web::Data::new(I18nBundles::new());
    
    // 注册全局 i18n 翻译器到 common 模块
    football_common::init_i18n_translator(i18n::translate);

    // 初始化 Sa-Token
    let sa_token = football_common::conf::init_sa_token(&redis_config)
        .await
        .expect("Sa-Token 初始化失败");
    let sa_token = web::Data::new(sa_token);
    log::info!("Sa-Token 初始化成功");

    let server_host = config.server.host.clone();
    let server_port = config.server.port;
    let bind_address = format!("{}:{}", server_host, server_port);

    log::info!("Business服务启动在: http://{}", bind_address);

    // 启动 HTTP 服务器
    HttpServer::new(move || {
        // 配置 CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(i18n.clone()) // 注入 i18n 到应用状态
            .app_data(sa_token.clone()) // 注入 Sa-Token 到应用状态
            .wrap(LanguageMiddleware) // 自动检测语言并注入请求
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .configure(routes::config)
    })
        .bind(&bind_address)?
        .run()
        .await
}
