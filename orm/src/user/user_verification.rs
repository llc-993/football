// 实名认证表 - 使用 rbatis CRUD 宏
use rbatis::crud;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

/// 实名认证实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVerification {
    pub id: Option<i64>,
    pub user_id: i64,
    pub real_name: String,
    pub id_card: String,
    pub id_card_front: Option<String>,
    pub id_card_back: Option<String>,
    /// 认证状态: 0-待审核, 1-已通过, 2-已拒绝
    pub verification_status: i8,
    /// 认证通过时间
    pub verified_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

// 使用 rbatis 的 crud 宏自动生成基础 CRUD 方法
crud!(UserVerification {});

impl UserVerification {
    /// 创建新的认证记录
    pub fn new(user_id: i64, real_name: String, id_card: String) -> Self {
        Self {
            id: None,
            user_id,
            real_name,
            id_card,
            id_card_front: None,
            id_card_back: None,
            verification_status: 0, // 默认待审核
            verified_at: None,
            created_at: None,
            updated_at: None,
        }
    }
}
