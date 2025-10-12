// Sa-Token 认证配置（简化版）
use sa_token_core::{SaTokenManager, SaTokenConfig};
use sa_token_storage_redis::RedisStorage;
use std::sync::Arc;
use sa_token_core::config::TokenStyle;
use super::redis_conf::RedisConfig;

/// Sa-Token 管理器类型
pub type SaTokenMgr = SaTokenManager;

/// 初始化 Sa-Token
/// 
/// # 参数
/// - `redis_config`: Redis 配置
/// 
/// # 返回
/// - Sa-Token 管理器实例
pub async fn init_sa_token(redis_config: &RedisConfig) -> Result<SaTokenMgr, String> {
    let redis_url = redis_config.build_url();
    
    let storage = RedisStorage::new(&redis_url, "sa-token:")
        .await
        .map_err(|e| format!("Sa-Token-redis 初始化失败: {:?}", e))?;

    let sa_token_manager = SaTokenConfig::builder()
        .token_name("Authorization")
        .token_style(TokenStyle::SimpleUuid)
        .timeout(86400)
        .active_timeout(86400)
        .storage(Arc::new(storage))
        .auto_renew(true)
        .is_share(false)
        .build();
    Ok(sa_token_manager)
}
