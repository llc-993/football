// 代理佣金配置表 - 使用 rbatis CRUD 宏
use rbatis::crud;
use rbatis::rbdc::datetime::DateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 代理佣金配置实体
/// 注意: commission_type 在数据库中存储为 ENUM('DEPOSIT', 'BET', 'PROFIT')
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCommissionConfig {
    pub id: Option<i64>,
    pub agent_id: i64,
    /// 佣金类型: DEPOSIT, BET, PROFIT
    pub commission_type: String,
    /// 佣金比例(%)
    pub commission_rate: Decimal,
    /// 最小金额
    pub min_amount: Decimal,
    /// 最大金额
    pub max_amount: Option<Decimal>,
    /// 是否生效
    pub is_active: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

// 使用 rbatis 的 crud 宏自动生成基础 CRUD 方法
crud!(AgentCommissionConfig {});

// 自定义查询方法
rbatis::impl_select!(AgentCommissionConfig{select_by_agent_id(agent_id: i64) -> Vec => "`where agent_id = #{agent_id}`"});
rbatis::impl_select!(AgentCommissionConfig{select_by_commission_type(commission_type: &str) -> Vec => "`where commission_type = #{commission_type}`"});
rbatis::impl_select!(AgentCommissionConfig{select_active_configs() -> Vec => "`where is_active = true`"});
rbatis::impl_select!(AgentCommissionConfig{select_by_agent_and_type(agent_id: i64, commission_type: &str) -> Option => "`where agent_id = #{agent_id} and commission_type = #{commission_type} limit 1`"});
rbatis::impl_select!(AgentCommissionConfig{select_by_rate_range(min_rate: &str, max_rate: &str) -> Vec => "`where commission_rate >= #{min_rate} and commission_rate <= #{max_rate}`"});

impl AgentCommissionConfig {
    /// 佣金类型常量
    pub const DEPOSIT: &'static str = "DEPOSIT";
    pub const BET: &'static str = "BET";
    pub const PROFIT: &'static str = "PROFIT";

    /// 创建新的佣金配置
    pub fn new(agent_id: i64, commission_type: String, commission_rate: Decimal) -> Self {
        Self {
            id: None,
            agent_id,
            commission_type,
            commission_rate,
            min_amount: Decimal::ZERO,
            max_amount: None,
            is_active: true,
            created_at: None,
            updated_at: None,
        }
    }
}
