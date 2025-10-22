// 业务数据类型定义
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

/// 用户注册请求
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
}

/// 用户登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 修改密码请求
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

/// 找回密码请求
#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub verification_code: String,
    pub new_password: String,
}

/// 用户详细信息响应（过滤敏感字段）
#[derive(Debug, Serialize)]
pub struct UserDetailResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub status: i8,
    pub account_type: i8,
    pub user_level: i32,
    pub is_verified: bool,
    pub is_agent: bool,
    pub trade_banned: bool,
    pub withdraw_limited: bool,
    pub account_limited: bool,
    pub last_login_at: Option<DateTime>,
    // 等级配置信息
    pub level_config: Option<UserLevelConfigResponse>,
}

/// 用户等级配置响应（前端需要的字段）
#[derive(Debug, Serialize)]
pub struct UserLevelConfigResponse {
    pub level_name: String,
    pub level_value: i32,
    pub daily_withdraw_limit: String,
    pub single_withdraw_limit: String,
    pub withdraw_fee_rate: String,
    pub min_withdraw_amount: String,
    pub max_withdraw_amount: String,
    pub deposit_fee_rate: String,
    pub min_deposit_amount: String,
    pub max_deposit_amount: String,
    pub bet_limit: String,
    pub single_bet_limit: String,
    pub daily_bet_limit: String,
    pub commission_rate: String,
    pub require_verification: bool,
    pub require_phone_verification: bool,
    pub require_email_verification: bool,
    pub description: Option<String>,
}

impl From<football_orm::user::user_level_config::UserLevelConfig> for UserLevelConfigResponse {
    fn from(config: football_orm::user::user_level_config::UserLevelConfig) -> Self {
        Self {
            level_name: config.level_name,
            level_value: config.level_value,
            daily_withdraw_limit: config.daily_withdraw_limit.to_string(),
            single_withdraw_limit: config.single_withdraw_limit.to_string(),
            withdraw_fee_rate: config.withdraw_fee_rate.to_string(),
            min_withdraw_amount: config.min_withdraw_amount.to_string(),
            max_withdraw_amount: config.max_withdraw_amount.to_string(),
            deposit_fee_rate: config.deposit_fee_rate.to_string(),
            min_deposit_amount: config.min_deposit_amount.to_string(),
            max_deposit_amount: config.max_deposit_amount.to_string(),
            bet_limit: config.bet_limit.to_string(),
            single_bet_limit: config.single_bet_limit.to_string(),
            daily_bet_limit: config.daily_bet_limit.to_string(),
            commission_rate: config.commission_rate.to_string(),
            require_verification: config.require_verification,
            require_phone_verification: config.require_phone_verification,
            require_email_verification: config.require_email_verification,
            description: config.description,
        }
    }
}

/// 转换用户为详细信息响应（过滤敏感字段）
pub fn user_to_detail_response(
    user: football_orm::user::users::User, 
    level_config: Option<football_orm::user::user_level_config::UserLevelConfig>
) -> UserDetailResponse {
    UserDetailResponse {
        id: user.id.unwrap_or(0),
        username: user.username,
        email: user.email,
        phone: user.phone,
        status: user.status,
        account_type: user.account_type,
        user_level: user.user_level,
        is_verified: user.is_verified,
        is_agent: user.is_agent,
        trade_banned: user.trade_banned,
        withdraw_limited: user.withdraw_limited,
        account_limited: user.account_limited,
        last_login_at: user.last_login_at,
        level_config: level_config.map(|c| c.into()),
    }
}
