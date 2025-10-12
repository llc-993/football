// MySQL 数据库连接管理
use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use once_cell::sync::OnceCell;
use football_common::{AppResult, conf::DbConfig};

/// 全局数据库实例
static DB: OnceCell<RBatis> = OnceCell::new();

pub async fn init_db(config: &DbConfig) -> AppResult<()> {
    let rb = RBatis::new();
    
    // 构建带连接池参数的 URL
    let url_with_pool = config.build_url_with_pool();
    
    // 使用 link 方法初始化连接池
    // rbatis 内部会自动创建和管理连接池
    rb.link(MysqlDriver {}, &url_with_pool).await?;
    
    DB.set(rb).map_err(|_| {
        football_common::AppError::DatabaseError("数据库已初始化".to_string())
    })?;
    
    log::info!(
        "MySQL 连接池初始化成功 - URL: {}, 最大连接数: {}",
        config.url,
        config.max_connections
    );
    Ok(())
}

/// 获取数据库实例
/// 
/// # Panics
/// 如果数据库未初始化则会 panic
pub fn get_db() -> &'static RBatis {
    DB.get().expect("数据库未初始化，请先调用 init_db() 或 init_db_with_config()")
}

/// 测试数据库连接
pub async fn test_connection() -> AppResult<bool> {
    let rb = get_db();
    let result = rb.query("SELECT 1", vec![]).await;
    match result {
        Ok(_) => {
            log::info!(" 数据库连接测试成功");
            Ok(true)
        }
        Err(e) => {
            log::error!("❌ 数据库连接测试失败: {}", e);
            Err(football_common::AppError::DatabaseError(e.to_string()))
        }
    }
}

/// 获取连接池状态（如果可用）
pub fn get_pool_status() -> String {
    let rb = get_db();
    format!("数据库连接池状态: {:?}", rb)
}
