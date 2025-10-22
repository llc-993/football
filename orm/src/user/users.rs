// 用户主表 - 使用 rbatis CRUD 宏
use rbatis::crud;
use rbatis::rbdc::datetime::DateTime;
use rbs::value;
use serde::{Deserialize, Serialize};

/// 用户实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub password_hash: String,
    /// 用户状态: 1-正常, 2-冻结, 3-禁用
    pub status: i8,
    /// 账号类型: 1-普通用户, 2-代理账号, 3-测试账号
    pub account_type: i8,
    /// 用户等级
    pub user_level: i32,
    /// 是否实名认证
    pub is_verified: bool,
    /// 是否为代理
    pub is_agent: bool,
    /// 禁止交易
    pub trade_banned: bool,
    /// 提现限制
    pub withdraw_limited: bool,
    /// 账号限制
    pub account_limited: bool,
    /// 上级代理ID
    pub agent_id: Option<i64>,
    /// 代理路径，如: 1,2,3
    pub agent_path: Option<String>,
    /// 代理层级
    pub agent_level: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    /// 最后登录时间
    pub last_login_at: Option<DateTime>,
}

// 使用 rbatis 的 crud 宏自动生成基础 CRUD 方法
// 这会生成: insert, update_by_column, delete_by_column, select_by_column 等方法
crud!(User {});

// 自定义查询方法
rbatis::impl_select!(User{select_by_id(id: i64) -> Option => "`where id = #{id} limit 1`"});
rbatis::impl_select!(User{select_by_username(username: &str) -> Option => "`where username = #{username} limit 1`"});
rbatis::impl_select!(User{select_by_email(email: &str) -> Option => "`where email = #{email} limit 1`"});
rbatis::impl_select!(User{select_by_phone(phone: &str) -> Option => "`where phone = #{phone} limit 1`"});
rbatis::impl_select!(User{select_by_agent_id(agent_id: i64) -> Vec => "`where agent_id = #{agent_id}`"});
rbatis::impl_select!(User{select_by_status(status: i8) -> Vec => "`where status = #{status}`"});
rbatis::impl_select!(User{select_by_account_type(account_type: i8) -> Vec => "`where account_type = #{account_type}`"});
rbatis::impl_select!(User{select_by_user_level(user_level: i32) -> Vec => "`where user_level = #{user_level}`"});
rbatis::impl_select!(User{select_agents() -> Vec => "`where is_agent = true`"});
rbatis::impl_select!(User{select_verified_users() -> Vec => "`where is_verified = true`"});
rbatis::impl_select!(User{select_trade_banned_users() -> Vec => "`where trade_banned = true`"});
rbatis::impl_select!(User{select_withdraw_limited_users() -> Vec => "`where withdraw_limited = true`"});
rbatis::impl_select!(User{select_account_limited_users() -> Vec => "`where account_limited = true`"});
rbatis::impl_select!(User{select_by_agent_path(agent_path: &str) -> Vec => "`where agent_path like #{agent_path}`"});
rbatis::impl_select!(User{select_by_agent_level(agent_level: i32) -> Vec => "`where agent_level = #{agent_level}`"});
rbatis::impl_select!(User{select_recent_login_users(days: i32) -> Vec => "`where last_login_at >= DATE_SUB(NOW(), INTERVAL #{days} DAY)`"});
rbatis::impl_select!(User{select_users_by_date_range(start_date: &str, end_date: &str) -> Vec => "`where created_at >= #{start_date} and created_at <= #{end_date}`"});

impl User {
    /// 创建新用户
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        Self {
            id: None,
            username,
            email,
            phone: None,
            password_hash,
            status: 1, // 默认正常状态
            account_type: 1, // 默认普通用户
            user_level: 1, // 默认等级1
            is_verified: false,
            is_agent: false,
            trade_banned: false,
            withdraw_limited: false,
            account_limited: false,
            agent_id: None,
            agent_path: None,
            agent_level: 0,
            created_at: None,
            updated_at: None,
            last_login_at: None,
        }
    }

    /// 获取所有用户
    pub async fn get_users() -> Result<Vec<Self>, rbatis::rbdc::Error> {
        use crate::db::get_db;
        let rb = get_db();
        Self::select_all(rb).await
    }

    /// 根据ID获取用户
    pub async fn get_user(user_id: i64) -> Result<Option<Self>, rbatis::rbdc::Error> {
        use crate::db::get_db;
        let rb = get_db();
        Self::select_by_id(rb, user_id).await
    }

    /// 根据用户名获取用户
    pub async fn get_user_by_username(username: &str) -> Result<Option<Self>, rbatis::rbdc::Error> {
        use crate::db::get_db;
        let rb = get_db();
        Self::select_by_username(rb, username).await
    }

    /// 更新最后登录时间
    pub async fn update_last_login_time(mut user: Self) -> Result<u64, rbatis::rbdc::Error> {
        use crate::db::get_db;
        use rbatis::rbdc::datetime::DateTime;
        
        let rb = get_db();
        let now = DateTime::now();
        
        // 更新最后登录时间
        user.last_login_at = Some(now);
        
        // 更新数据库
        let result = Self::update_by_map(rb, &user, value!{"id":&user.id}).await?;
        Ok(result.rows_affected)
    }
}
