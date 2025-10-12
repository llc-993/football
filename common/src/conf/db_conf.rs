// MySQL 数据库配置
use serde::{Deserialize, Serialize};

/// MySQL 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    /// 数据库连接 URL
    pub url: String,
    /// 连接池最大连接数
    pub max_connections: u64,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: "mysql://root:password@localhost:3306/football".to_string(),
            max_connections: 10,
        }
    }
}

impl DbConfig {
    /// 创建新的数据库配置
    pub fn new(url: String, max_connections: u64) -> Self {
        Self {
            url,
            max_connections,
        }
    }

    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        Self {
            url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "mysql://root:password@localhost:3306/football".to_string()),
            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
        }
    }

    /// 构建带连接池参数的数据库 URL
    pub fn build_url_with_pool(&self) -> String {
        if self.url.contains('?') {
            format!("{}&max_connections={}", self.url, self.max_connections)
        } else {
            format!("{}?max_connections={}", self.url, self.max_connections)
        }
    }
}

