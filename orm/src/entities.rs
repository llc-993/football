// 数据库实体定义 - 使用 rbatis 的宏定义实体
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 示例实体：用户表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 可以添加更多实体定义
// 例如：
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Team {
//     pub id: i64,
//     pub name: String,
//     pub created_at: DateTime<Utc>,
// }

