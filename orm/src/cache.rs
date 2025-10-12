// Redis 缓存连接管理
use redis::{Client, Connection, RedisResult, AsyncCommands};
use redis::aio::ConnectionManager;
use once_cell::sync::OnceCell;
use football_common::{AppResult, conf::RedisConfig};

/// 全局 Redis 客户端
static REDIS_CLIENT: OnceCell<Client> = OnceCell::new();

/// 初始化 Redis 连接
/// 
/// # 参数
/// * `config` - Redis 配置
pub fn init_redis(config: &RedisConfig) -> AppResult<()> {
    let url = config.build_url();
    let client = Client::open(url.as_str())
        .map_err(|e| football_common::AppError::RedisError(e.to_string()))?;
    
    REDIS_CLIENT.set(client).map_err(|_| {
        football_common::AppError::RedisError("Redis已初始化".to_string())
    })?;
    
    // 隐藏密码信息
    let safe_url = if config.password.is_some() {
        format!("redis://{}:{}/{} (有密码)", config.host, config.port, config.database)
    } else {
        format!("redis://{}:{}/{}", config.host, config.port, config.database)
    };
    
    log::info!(
        "Redis连接初始化成功 - {}, 连接池大小: {}",
        safe_url,
        config.pool_size
    );
    Ok(())
}

/// 获取 Redis 客户端
pub fn get_redis_client() -> &'static Client {
    REDIS_CLIENT.get().expect("Redis未初始化")
}

/// 获取 Redis 连接
pub fn get_redis_connection() -> RedisResult<Connection> {
    get_redis_client().get_connection()
}

/// 获取异步 Redis 连接
pub async fn get_async_connection() -> AppResult<ConnectionManager> {
    let client = get_redis_client();
    ConnectionManager::new(client.clone())
        .await
        .map_err(|e| football_common::AppError::RedisError(e.to_string()))
}

/// Redis 缓存操作封装
pub struct RedisCache;

impl RedisCache {
    /// 设置缓存
    pub async fn set(key: &str, value: &str, expire_seconds: usize) -> AppResult<()> {
        let mut conn = get_async_connection().await?;
        conn.set_ex::<_, _, ()>(key, value, expire_seconds as u64)
            .await
            .map_err(|e| football_common::AppError::RedisError(e.to_string()))?;
        Ok(())
    }

    /// 获取缓存
    pub async fn get(key: &str) -> AppResult<Option<String>> {
        let mut conn = get_async_connection().await?;
        let result: RedisResult<String> = conn.get(key).await;
        match result {
            Ok(value) => Ok(Some(value)),
            Err(e) => {
                if e.kind() == redis::ErrorKind::TypeError {
                    Ok(None)
                } else {
                    Err(football_common::AppError::RedisError(e.to_string()))
                }
            }
        }
    }

    /// 删除缓存
    pub async fn delete(key: &str) -> AppResult<()> {
        let mut conn = get_async_connection().await?;
        conn.del::<_, ()>(key)
            .await
            .map_err(|e| football_common::AppError::RedisError(e.to_string()))?;
        Ok(())
    }

    /// 判断键是否存在
    pub async fn exists(key: &str) -> AppResult<bool> {
        let mut conn = get_async_connection().await?;
        conn.exists(key)
            .await
            .map_err(|e| football_common::AppError::RedisError(e.to_string()))
    }

    /// 设置过期时间
    pub async fn expire(key: &str, seconds: usize) -> AppResult<()> {
        let mut conn = get_async_connection().await?;
        conn.expire::<_, ()>(key, seconds as i64)
            .await
            .map_err(|e| football_common::AppError::RedisError(e.to_string()))?;
        Ok(())
    }
}

/// 测试 Redis 连接
pub async fn test_connection() -> AppResult<bool> {
    let mut conn = get_async_connection().await?;
    let result: RedisResult<String> = redis::cmd("PING").query_async(&mut conn).await;
    match result {
        Ok(_) => {
            log::info!("Redis连接测试成功");
            Ok(true)
        }
        Err(e) => {
            log::error!("Redis连接测试失败: {}", e);
            Err(football_common::AppError::RedisError(e.to_string()))
        }
    }
}

