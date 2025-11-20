// 爬虫配置
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiderConfig {
    /// 用户代理
    pub user_agent: String,
    /// 请求超时时间（秒）
    pub timeout: u64,
    /// 请求间隔（毫秒）
    pub delay_ms: u64,
    /// 最大重试次数
    pub max_retries: u32,
    /// 是否启用代理
    pub use_proxy: bool,
    /// 代理地址（可选）
    pub proxy_url: Option<String>,
}

impl Default for SpiderConfig {
    fn default() -> Self {
        Self {
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            timeout: 30,
            delay_ms: 1000,
            max_retries: 3,
            use_proxy: false,
            proxy_url: None,
        }
    }
}

impl SpiderConfig {
    /// 从配置文件加载配置
    pub fn load() -> anyhow::Result<Self> {
        // 尝试从环境变量或配置文件加载
        let config_path = std::env::var("SPIDER_CONFIG")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("config.toml"));
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: SpiderConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            // 使用默认配置
            Ok(Self::default())
        }
    }
}

