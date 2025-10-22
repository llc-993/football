// 用户限制记录表 - 使用 rbatis CRUD 宏
use rbatis::crud;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

/// 用户限制记录实体
/// 注意: restriction_type 在数据库中存储为 ENUM('TRADE_BAN', 'WITHDRAW_LIMIT', 'ACCOUNT_LIMIT')
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRestriction {
    pub id: Option<i64>,
    pub user_id: i64,
    /// 限制类型: TRADE_BAN, WITHDRAW_LIMIT, ACCOUNT_LIMIT
    pub restriction_type: String,
    /// 限制原因
    pub reason: Option<String>,
    /// 操作员ID
    pub operator_id: Option<i64>,
    /// 是否生效
    pub is_active: bool,
    /// 过期时间
    pub expires_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

// 使用 rbatis 的 crud 宏自动生成基础 CRUD 方法
crud!(UserRestriction {});

impl UserRestriction {
    /// 限制类型常量
    pub const TRADE_BAN: &'static str = "TRADE_BAN";
    pub const WITHDRAW_LIMIT: &'static str = "WITHDRAW_LIMIT";
    pub const ACCOUNT_LIMIT: &'static str = "ACCOUNT_LIMIT";

    /// 创建新的限制记录
    pub fn new(user_id: i64, restriction_type: String, reason: Option<String>, operator_id: Option<i64>) -> Self {
        Self {
            id: None,
            user_id,
            restriction_type,
            reason,
            operator_id,
            is_active: true,
            expires_at: None,
            created_at: None,
            updated_at: None,
        }
    }
}
