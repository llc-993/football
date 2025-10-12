use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, Environment, File, FileFormat};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub log: LogConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

impl AppConfig {
    /// 从配置文件加载配置
    pub fn from_file(config_path: &str) -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            // 加载默认配置
            .add_source(File::with_name(config_path).required(false))
            // 加载环境特定配置
            .add_source(File::with_name(&format!("{}.{}", config_path, run_mode)).required(false))
            // 从环境变量加载配置（前缀为 APP_）
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        config.try_deserialize()
    }

    /// 从嵌入的配置内容加载（支持编译时嵌入）
    pub fn from_embedded(default_config: &str, prod_config: Option<&str>) -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            // 加载嵌入的默认配置
            .add_source(File::from_str(default_config, FileFormat::Toml));

        // 如果是生产环境且提供了生产配置，加载生产配置
        if run_mode == "production" {
            if let Some(prod_cfg) = prod_config {
                builder = builder.add_source(File::from_str(prod_cfg, FileFormat::Toml));
            }
        }

        // 从环境变量加载配置（优先级最高）
        let config = builder
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        config.try_deserialize()
    }

    /// 智能加载配置：优先从文件加载，如果失败则从嵌入资源加载
    pub fn from_file_or_embedded(
        config_path: &str,
        default_config: &str,
        prod_config: Option<&str>,
    ) -> Result<Self, ConfigError> {
        // 优先从文件系统加载
        match Self::from_file(config_path) {
            Ok(config) => {
                log::debug!("从文件系统加载配置: {}", config_path);
                Ok(config)
            }
            Err(e) => {
                log::info!("文件系统加载配置失败: {}，使用嵌入配置", e);
                Self::from_embedded(default_config, prod_config)
            }
        }
    }

    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        Ok(AppConfig {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "mysql://root:password@localhost:3306/football".to_string()),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            redis: RedisConfig {
                url: env::var("REDIS_URL")
                    .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
                pool_size: env::var("REDIS_POOL_SIZE")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            log: LogConfig {
                level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            },
        })
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
            },
            database: DatabaseConfig {
                url: "mysql://root:password@localhost:3306/football".to_string(),
                max_connections: 10,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                pool_size: 10,
            },
            log: LogConfig {
                level: "info".to_string(),
            },
        }
    }
}

