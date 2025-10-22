// 用户等级配置表 - 使用 rbatis CRUD 宏
use rbatis::crud;
use rbatis::rbdc::datetime::DateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 用户等级配置实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLevelConfig {
    pub id: Option<i64>,
    /// 等级名称
    pub level_name: String,
    /// 等级值（数字越大等级越高）
    pub level_value: i32,
    /// 每天提现额度
    pub daily_withdraw_limit: Decimal,
    /// 单次提现限额
    pub single_withdraw_limit: Decimal,
    /// 提现手续费率（%）
    pub withdraw_fee_rate: Decimal,
    /// 最低提现金额
    pub min_withdraw_amount: Decimal,
    /// 最高提现金额
    pub max_withdraw_amount: Decimal,
    /// 充值手续费率（%）
    pub deposit_fee_rate: Decimal,
    /// 最低充值金额
    pub min_deposit_amount: Decimal,
    /// 最高充值金额
    pub max_deposit_amount: Decimal,
    /// 投注限额
    pub bet_limit: Decimal,
    /// 单次投注限额
    pub single_bet_limit: Decimal,
    /// 每日投注限额
    pub daily_bet_limit: Decimal,
    /// 佣金比例（%）
    pub commission_rate: Decimal,
    /// 是否需要实名认证
    pub require_verification: bool,
    /// 是否需要手机验证
    pub require_phone_verification: bool,
    /// 是否需要邮箱验证
    pub require_email_verification: bool,
    /// 等级描述
    pub description: Option<String>,
    /// 是否启用
    pub is_active: bool,
    /// 排序权重（数字越大越靠前）
    pub sort_order: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

// 使用 rbatis 的 crud 宏自动生成基础 CRUD 方法
crud!(UserLevelConfig {});

// 自定义查询方法
rbatis::impl_select!(UserLevelConfig{select_by_level_value(level_value: i32) -> Option => "`where level_value = #{level_value} limit 1`"});
rbatis::impl_select!(UserLevelConfig{select_by_level_name(level_name: &str) -> Option => "`where level_name = #{level_name} limit 1`"});
rbatis::impl_select!(UserLevelConfig{select_active_levels() -> Vec => "`where is_active = true order by sort_order desc`"});
rbatis::impl_select!(UserLevelConfig{select_by_min_withdraw_limit(min_limit: &str) -> Vec => "`where daily_withdraw_limit >= #{min_limit}`"});
rbatis::impl_select!(UserLevelConfig{select_by_max_withdraw_limit(max_limit: &str) -> Vec => "`where daily_withdraw_limit <= #{max_limit}`"});
rbatis::impl_select!(UserLevelConfig{select_by_fee_rate_range(min_rate: &str, max_rate: &str) -> Vec => "`where withdraw_fee_rate >= #{min_rate} and withdraw_fee_rate <= #{max_rate}`"});

impl UserLevelConfig {
    /// 创建新的等级配置
    pub fn new(
        level_name: String,
        level_value: i32,
        daily_withdraw_limit: Decimal,
        withdraw_fee_rate: Decimal,
    ) -> Self {
        Self {
            id: None,
            level_name,
            level_value,
            daily_withdraw_limit,
            single_withdraw_limit: daily_withdraw_limit, // 默认单次限额等于日限额
            withdraw_fee_rate,
            min_withdraw_amount: Decimal::new(100, 0), // 默认最低100元
            max_withdraw_amount: daily_withdraw_limit,
            deposit_fee_rate: Decimal::ZERO, // 默认充值无手续费
            min_deposit_amount: Decimal::new(10, 0), // 默认最低10元
            max_deposit_amount: Decimal::new(100000, 0), // 默认最高10万
            bet_limit: Decimal::new(10000, 0), // 默认投注限额1万
            single_bet_limit: Decimal::new(1000, 0), // 默认单次投注限额1千
            daily_bet_limit: Decimal::new(50000, 0), // 默认日投注限额5万
            commission_rate: Decimal::ZERO, // 默认无佣金
            require_verification: true, // 默认需要实名认证
            require_phone_verification: true, // 默认需要手机验证
            require_email_verification: false, // 默认不需要邮箱验证
            description: None,
            is_active: true,
            sort_order: level_value,
            created_at: None,
            updated_at: None,
        }
    }

    /// 创建VIP等级配置
    pub fn new_vip(level_value: i32, daily_withdraw_limit: Decimal) -> Self {
        let mut config = Self::new(
            format!("VIP{}", level_value),
            level_value,
            daily_withdraw_limit,
            Decimal::new(5, 2), // 0.05% 手续费
        );
        config.commission_rate = Decimal::new(level_value as i64 * 5, 2); // 等级越高佣金越高
        config.require_verification = true;
        config.require_phone_verification = true;
        config.require_email_verification = true;
        config.description = Some(format!("VIP{}等级用户，享受更高限额和更低手续费", level_value));
        config
    }

    /// 创建代理等级配置
    pub fn new_agent(level_value: i32, daily_withdraw_limit: Decimal) -> Self {
        let mut config = Self::new(
            format!("代理{}", level_value),
            level_value,
            daily_withdraw_limit,
            Decimal::new(2, 2), // 0.02% 手续费
        );
        config.commission_rate = Decimal::new(level_value as i64 * 10, 2); // 代理佣金更高
        config.require_verification = true;
        config.require_phone_verification = true;
        config.require_email_verification = true;
        config.description = Some(format!("代理{}等级，享受代理佣金和更高限额", level_value));
        config
    }

    /// 创建测试账号配置
    pub fn new_test() -> Self {
        let mut config = Self::new(
            "测试账号".to_string(),
            0, // 测试账号等级为0
            Decimal::new(1000, 0), // 测试账号日限额1000元
            Decimal::ZERO, // 测试账号无手续费
        );
        config.require_verification = false;
        config.require_phone_verification = false;
        config.require_email_verification = false;
        config.description = Some("测试账号，用于开发和测试环境".to_string());
        config
    }
}
