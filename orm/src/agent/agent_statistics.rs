// 代理统计表 - 使用 rbatis CRUD 宏
use rbatis::crud;
use rbatis::rbdc::datetime::DateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 代理统计实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatistics {
    pub id: Option<i64>,
    /// 上级代理用户id
    pub agent_id: i64,
    /// 当前用户ID
    pub user_id: i64,
    /// 总充值金额
    pub total_deposit: Decimal,
    /// 充值次数
    pub deposit_count: i32,
    /// 总提现金额
    pub total_withdraw: Decimal,
    /// 提现次数
    pub withdraw_count: i32,
    /// 总投注金额
    pub total_bet_amount: Decimal,
    /// 投注次数
    pub bet_count: i32,
    /// 总盈利金额
    pub total_profit: Decimal,
    /// 总亏损金额
    pub total_loss: Decimal,
    /// 净利润
    pub net_profit: Decimal,
    /// 总佣金
    pub total_commission: Decimal,
    /// 佣金比例(%)
    pub commission_rate: Decimal,
    /// 统计日期
    pub stat_date: String, // DATE 类型，使用 String 存储 YYYY-MM-DD 格式
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

// 使用 rbatis 的 crud 宏自动生成基础 CRUD 方法
crud!(AgentStatistics {});

// 自定义查询方法
rbatis::impl_select!(AgentStatistics{select_by_agent_id(agent_id: i64) -> Vec => "`where agent_id = #{agent_id}`"});
rbatis::impl_select!(AgentStatistics{select_by_user_id(user_id: i64) -> Vec => "`where user_id = #{user_id}`"});
rbatis::impl_select!(AgentStatistics{select_by_date(stat_date: &str) -> Vec => "`where stat_date = #{stat_date}`"});
rbatis::impl_select!(AgentStatistics{select_by_date_range(start_date: &str, end_date: &str) -> Vec => "`where stat_date >= #{start_date} and stat_date <= #{end_date}`"});
rbatis::impl_select!(AgentStatistics{select_by_agent_and_date(agent_id: i64, stat_date: &str) -> Option => "`where agent_id = #{agent_id} and stat_date = #{stat_date} limit 1`"});
rbatis::impl_select!(AgentStatistics{select_profitable_agents() -> Vec => "`where net_profit > 0`"});
rbatis::impl_select!(AgentStatistics{select_losing_agents() -> Vec => "`where net_profit < 0`"});
rbatis::impl_select!(AgentStatistics{select_top_agents_by_profit(limit: i32) -> Vec => "`order by net_profit desc limit #{limit}`"});
rbatis::impl_select!(AgentStatistics{select_top_agents_by_commission(limit: i32) -> Vec => "`order by total_commission desc limit #{limit}`"});

impl AgentStatistics {
    /// 创建新的统计记录
    pub fn new(agent_id: i64, user_id: i64, stat_date: String) -> Self {
        Self {
            id: None,
            agent_id,
            user_id,
            total_deposit: Decimal::ZERO,
            deposit_count: 0,
            total_withdraw: Decimal::ZERO,
            withdraw_count: 0,
            total_bet_amount: Decimal::ZERO,
            bet_count: 0,
            total_profit: Decimal::ZERO,
            total_loss: Decimal::ZERO,
            net_profit: Decimal::ZERO,
            total_commission: Decimal::ZERO,
            commission_rate: Decimal::ZERO,
            stat_date,
            created_at: None,
            updated_at: None,
        }
    }
}
