pub mod repositories;
pub mod db;
pub mod cache;
pub mod user;
pub mod agent;

// 重新导出 common 模块
pub use football_common::{AppError, AppResult};

// 重新导出配置
pub use football_common::conf::{DbConfig, RedisConfig};

// 重新导出数据库相关
pub use rbatis;
pub use rbdc;
pub use redis;
