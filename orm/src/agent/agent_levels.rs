// 代理层级表 - 使用 rbatis CRUD 宏
use rbatis::crud;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

/// 代理层级实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentLevel {
    pub id: Option<i64>,
    /// 代理ID
    pub agent_id: i64,
    /// 层级
    pub level: i32,
    /// 该层级的用户ID
    pub user_id: i64,
    pub created_at: Option<DateTime>,
}

// 使用 rbatis 的 crud 宏自动生成基础 CRUD 方法
crud!(AgentLevel {});

impl AgentLevel {
    /// 创建新的层级记录
    pub fn new(agent_id: i64, level: i32, user_id: i64) -> Self {
        Self {
            id: None,
            agent_id,
            level,
            user_id,
            created_at: None,
        }
    }
}
