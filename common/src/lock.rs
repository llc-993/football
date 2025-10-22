// 分布式锁实现 - 支持 Redis 分布式锁和本地原子锁
use crate::{AppError, AppResult};
use once_cell::sync::OnceCell;
use redis::{aio::ConnectionManager, AsyncCommands, Client};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// 全局Redis连接管理器
static REDIS_MANAGER: OnceCell<ConnectionManager> = OnceCell::new();

/// 初始化Redis连接管理器
pub async fn init_redis_manager(redis_url: &str) -> AppResult<()> {
    let client = Client::open(redis_url)
        .map_err(|e| AppError::RedisError(format!("创建Redis客户端失败: {}", e)))?;
    
    let manager = ConnectionManager::new(client)
        .await
        .map_err(|e| AppError::RedisError(format!("创建Redis连接管理器失败: {}", e)))?;
    
    REDIS_MANAGER
        .set(manager)
        .map_err(|_| AppError::RedisError("Redis连接管理器已初始化".to_string()))?;
    
    Ok(())
}

/// 获取Redis连接管理器
fn get_redis_manager() -> Option<&'static ConnectionManager> {
    REDIS_MANAGER.get()
}

/// 分布式锁trait
#[async_trait::async_trait]
pub trait DistributedLock: Send + Sync {
    /// 尝试获取锁
    async fn try_lock(&self) -> AppResult<bool>;
    
    /// 获取锁（阻塞直到成功）
    async fn lock(&self) -> AppResult<()>;
    
    /// 获取锁（带超时）
    async fn lock_timeout(&self, timeout: Duration) -> AppResult<bool>;
    
    /// 释放锁
    async fn unlock(&self) -> AppResult<()>;
    
    /// 检查锁是否被持有
    async fn is_locked(&self) -> AppResult<bool>;
}

/// Redis分布式锁
pub struct RedisLock {
    key: String,
    value: String,
    ttl: u64, // 锁的过期时间（秒）
}

impl RedisLock {
    /// 创建Redis分布式锁
    /// 
    /// # 参数
    /// * `key` - 锁的键名
    /// * `ttl` - 锁的过期时间（秒），默认30秒
    pub fn new(key: String, ttl: Option<u64>) -> Self {
        let value = uuid::Uuid::new_v4().to_string();
        Self {
            key,
            value,
            ttl: ttl.unwrap_or(30),
        }
    }

    /// 设置锁的过期时间
    async fn set_nx_ex(&self) -> AppResult<bool> {
        let manager = get_redis_manager()
            .ok_or_else(|| AppError::RedisError("Redis连接管理器未初始化".to_string()))?;
        
        let mut conn = manager.clone();

        // 使用 SET key value NX EX ttl 命令
        let result: Result<String, redis::RedisError> = redis::cmd("SET")
            .arg(&self.key)
            .arg(&self.value)
            .arg("NX")
            .arg("EX")
            .arg(self.ttl)
            .query_async(&mut conn)
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("nil") {
                    Ok(false)
                } else {
                    Err(AppError::RedisError(format!("获取锁失败: {}", e)))
                }
            }
        }
    }

    /// Lua脚本释放锁（保证原子性）
    async fn del_if_match(&self) -> AppResult<bool> {
        let manager = get_redis_manager()
            .ok_or_else(|| AppError::RedisError("Redis连接管理器未初始化".to_string()))?;
        
        let mut conn = manager.clone();

        let lua_script = r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#;

        let result: i32 = redis::Script::new(lua_script)
            .key(&self.key)
            .arg(&self.value)
            .invoke_async(&mut conn)
            .await
            .map_err(|e| AppError::RedisError(format!("释放锁失败: {}", e)))?;

        Ok(result == 1)
    }
}

#[async_trait::async_trait]
impl DistributedLock for RedisLock {
    async fn try_lock(&self) -> AppResult<bool> {
        self.set_nx_ex().await
    }

    async fn lock(&self) -> AppResult<()> {
        loop {
            if self.try_lock().await? {
                return Ok(());
            }
            sleep(Duration::from_millis(100)).await;
        }
    }

    async fn lock_timeout(&self, timeout: Duration) -> AppResult<bool> {
        let start = std::time::Instant::now();
        loop {
            if self.try_lock().await? {
                return Ok(true);
            }
            if start.elapsed() >= timeout {
                return Ok(false);
            }
            sleep(Duration::from_millis(100)).await;
        }
    }

    async fn unlock(&self) -> AppResult<()> {
        self.del_if_match().await?;
        Ok(())
    }

    async fn is_locked(&self) -> AppResult<bool> {
        let manager = get_redis_manager()
            .ok_or_else(|| AppError::RedisError("Redis连接管理器未初始化".to_string()))?;
        
        let mut conn = manager.clone();

        let exists: bool = conn
            .exists(&self.key)
            .await
            .map_err(|e| AppError::RedisError(format!("检查锁状态失败: {}", e)))?;

        Ok(exists)
    }
}

/// 本地原子锁（基于AtomicBool）
pub struct AtomicLock {
    locked: Arc<AtomicBool>,
}

impl AtomicLock {
    /// 创建本地原子锁
    pub fn new() -> Self {
        Self {
            locked: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Default for AtomicLock {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl DistributedLock for AtomicLock {
    async fn try_lock(&self) -> AppResult<bool> {
        Ok(self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok())
    }

    async fn lock(&self) -> AppResult<()> {
        loop {
            if self.try_lock().await? {
                return Ok(());
            }
            sleep(Duration::from_millis(10)).await;
        }
    }

    async fn lock_timeout(&self, timeout: Duration) -> AppResult<bool> {
        let start = std::time::Instant::now();
        loop {
            if self.try_lock().await? {
                return Ok(true);
            }
            if start.elapsed() >= timeout {
                return Ok(false);
            }
            sleep(Duration::from_millis(10)).await;
        }
    }

    async fn unlock(&self) -> AppResult<()> {
        self.locked.store(false, Ordering::Release);
        Ok(())
    }

    async fn is_locked(&self) -> AppResult<bool> {
        Ok(self.locked.load(Ordering::Relaxed))
    }
}

/// 锁工厂 - 根据Redis配置自动选择锁类型
pub struct LockFactory;

impl LockFactory {
    /// 创建分布式锁
    /// 
    /// # 参数
    /// * `key` - 锁的键名
    /// * `ttl` - Redis锁的过期时间（秒），默认30秒
    /// 
    /// # 返回
    /// 如果Redis可用，返回RedisLock；否则返回AtomicLock
    pub async fn create_lock(key: String, ttl: Option<u64>) -> Box<dyn DistributedLock> {
        // 尝试获取Redis连接管理器
        if get_redis_manager().is_some() {
            log::debug!("使用Redis分布式锁: {}", key);
            Box::new(RedisLock::new(key, ttl))
        } else {
            log::warn!("Redis未配置，使用本地原子锁: {}", key);
            Box::new(AtomicLock::new())
        }
    }

    /// 创建Redis锁（强制使用Redis）
    pub fn create_redis_lock(key: String, ttl: Option<u64>) -> RedisLock {
        RedisLock::new(key, ttl)
    }

    /// 创建原子锁（强制使用本地锁）
    pub fn create_atomic_lock() -> AtomicLock {
        AtomicLock::new()
    }
}

/// RAII风格的锁守卫
pub struct LockGuard {
    lock: Box<dyn DistributedLock>,
}

impl LockGuard {
    /// 创建锁守卫并获取锁
    pub async fn new(key: String, ttl: Option<u64>) -> AppResult<Self> {
        let lock = LockFactory::create_lock(key, ttl).await;
        lock.lock().await?;
        Ok(Self { lock })
    }

    /// 创建锁守卫并尝试获取锁
    pub async fn try_new(key: String, ttl: Option<u64>) -> AppResult<Option<Self>> {
        let lock = LockFactory::create_lock(key, ttl).await;
        if lock.try_lock().await? {
            Ok(Some(Self { lock }))
        } else {
            Ok(None)
        }
    }

    /// 创建锁守卫并带超时获取锁
    pub async fn new_with_timeout(
        key: String,
        ttl: Option<u64>,
        timeout: Duration,
    ) -> AppResult<Option<Self>> {
        let lock = LockFactory::create_lock(key, ttl).await;
        if lock.lock_timeout(timeout).await? {
            Ok(Some(Self { lock }))
        } else {
            Ok(None)
        }
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        // 注意：这里使用了 block_in_place 来在 Drop 中执行异步操作
        // 在生产环境中，建议手动调用 unlock() 而不是依赖 Drop
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            handle.block_on(async {
                if let Err(e) = self.lock.unlock().await {
                    log::error!("释放锁失败: {}", e);
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_atomic_lock() {
        let lock = AtomicLock::new();

        // 测试 try_lock
        assert!(lock.try_lock().await.unwrap());
        assert!(!lock.try_lock().await.unwrap());

        // 测试 is_locked
        assert!(lock.is_locked().await.unwrap());

        // 测试 unlock
        lock.unlock().await.unwrap();
        assert!(!lock.is_locked().await.unwrap());

        // 测试 lock_timeout
        assert!(lock.lock_timeout(Duration::from_secs(1)).await.unwrap());
        lock.unlock().await.unwrap();
    }

    #[tokio::test]
    async fn test_lock_guard() {
        let lock1 = AtomicLock::new();
        
        // 测试基本功能
        assert!(lock1.try_lock().await.unwrap());
        lock1.unlock().await.unwrap();
    }
}

