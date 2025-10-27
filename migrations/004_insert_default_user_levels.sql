-- 插入默认用户等级配置数据

-- 测试账号
INSERT INTO user_level_config (
    level_name, level_value, daily_withdraw_limit, single_withdraw_limit, 
    withdraw_fee_rate, min_withdraw_amount, max_withdraw_amount,
    deposit_fee_rate, min_deposit_amount, max_deposit_amount,
    bet_limit, single_bet_limit, daily_bet_limit, commission_rate,
    require_verification, require_phone_verification, require_email_verification,
    description, is_active, sort_order
) VALUES (
    '测试账号', 0, 1000.00, 1000.00, 0.00, 10.00, 1000.00, 0.00, 10.00, 1000.00, 
    1000.00, 100.00, 1000.00, 0.00, FALSE, FALSE, FALSE, 
    '测试账号，用于开发和测试环境', TRUE, 0
);

-- 普通用户
INSERT INTO user_level_config (
    level_name, level_value, daily_withdraw_limit, single_withdraw_limit, 
    withdraw_fee_rate, min_withdraw_amount, max_withdraw_amount,
    deposit_fee_rate, min_deposit_amount, max_deposit_amount,
    bet_limit, single_bet_limit, daily_bet_limit, commission_rate,
    require_verification, require_phone_verification, require_email_verification,
    description, is_active, sort_order
) VALUES (
    '普通用户', 1, 5000.00, 2000.00, 0.10, 100.00, 5000.00, 0.00, 10.00, 50000.00, 
    10000.00, 1000.00, 50000.00, 0.00, TRUE, TRUE, FALSE, 
    '普通用户等级，基础限额和手续费', TRUE, 1
);

-- 银牌用户
INSERT INTO user_level_config (
    level_name, level_value, daily_withdraw_limit, single_withdraw_limit, 
    withdraw_fee_rate, min_withdraw_amount, max_withdraw_amount,
    deposit_fee_rate, min_deposit_amount, max_deposit_amount,
    bet_limit, single_bet_limit, daily_bet_limit, commission_rate,
    require_verification, require_phone_verification, require_email_verification,
    description, is_active, sort_order
) VALUES (
    '银牌用户', 2, 10000.00, 5000.00, 0.08, 100.00, 10000.00, 0.00, 10.00, 100000.00, 
    20000.00, 2000.00, 100000.00, 0.00, TRUE, TRUE, TRUE, 
    '银牌用户，享受更高限额和更低手续费', TRUE, 2
);

-- 金牌用户
INSERT INTO user_level_config (
    level_name, level_value, daily_withdraw_limit, single_withdraw_limit, 
    withdraw_fee_rate, min_withdraw_amount, max_withdraw_amount,
    deposit_fee_rate, min_deposit_amount, max_deposit_amount,
    bet_limit, single_bet_limit, daily_bet_limit, commission_rate,
    require_verification, require_phone_verification, require_email_verification,
    description, is_active, sort_order
) VALUES (
    '金牌用户', 3, 20000.00, 10000.00, 0.05, 100.00, 20000.00, 0.00, 10.00, 200000.00, 
    50000.00, 5000.00, 200000.00, 0.00, TRUE, TRUE, TRUE, 
    '金牌用户，享受更高限额和更低手续费', TRUE, 3
);

-- 钻石用户
INSERT INTO user_level_config (
    level_name, level_value, daily_withdraw_limit, single_withdraw_limit, 
    withdraw_fee_rate, min_withdraw_amount, max_withdraw_amount,
    deposit_fee_rate, min_deposit_amount, max_deposit_amount,
    bet_limit, single_bet_limit, daily_bet_limit, commission_rate,
    require_verification, require_phone_verification, require_email_verification,
    description, is_active, sort_order
) VALUES (
    '钻石用户', 4, 50000.00, 20000.00, 0.03, 100.00, 50000.00, 0.00, 10.00, 500000.00, 
    100000.00, 10000.00, 500000.00, 0.00, TRUE, TRUE, TRUE, 
    '钻石用户，享受最高限额和最低手续费', TRUE, 4
);

-- 代理1级
INSERT INTO user_level_config (
    level_name, level_value, daily_withdraw_limit, single_withdraw_limit, 
    withdraw_fee_rate, min_withdraw_amount, max_withdraw_amount,
    deposit_fee_rate, min_deposit_amount, max_deposit_amount,
    bet_limit, single_bet_limit, daily_bet_limit, commission_rate,
    require_verification, require_phone_verification, require_email_verification,
    description, is_active, sort_order
) VALUES (
    '代理1级', 10, 10000.00, 5000.00, 0.02, 100.00, 10000.00, 0.00, 10.00, 100000.00, 
    20000.00, 2000.00, 100000.00, 5.00, TRUE, TRUE, TRUE, 
    '代理1级，享受代理佣金和更高限额', TRUE, 10
);

-- 代理2级
INSERT INTO user_level_config (
    level_name, level_value, daily_withdraw_limit, single_withdraw_limit, 
    withdraw_fee_rate, min_withdraw_amount, max_withdraw_amount,
    deposit_fee_rate, min_deposit_amount, max_deposit_amount,
    bet_limit, single_bet_limit, daily_bet_limit, commission_rate,
    require_verification, require_phone_verification, require_email_verification,
    description, is_active, sort_order
) VALUES (
    '代理2级', 11, 20000.00, 10000.00, 0.02, 100.00, 20000.00, 0.00, 10.00, 200000.00, 
    50000.00, 5000.00, 200000.00, 8.00, TRUE, TRUE, TRUE, 
    '代理2级，享受更高代理佣金和限额', TRUE, 11
);

-- 代理3级
INSERT INTO user_level_config (
    level_name, level_value, daily_withdraw_limit, single_withdraw_limit, 
    withdraw_fee_rate, min_withdraw_amount, max_withdraw_amount,
    deposit_fee_rate, min_deposit_amount, max_deposit_amount,
    bet_limit, single_bet_limit, daily_bet_limit, commission_rate,
    require_verification, require_phone_verification, require_email_verification,
    description, is_active, sort_order
) VALUES (
    '代理3级', 12, 50000.00, 20000.00, 0.02, 100.00, 50000.00, 0.00, 10.00, 500000.00, 
    100000.00, 10000.00, 500000.00, 12.00, TRUE, TRUE, TRUE, 
    '代理3级，享受最高代理佣金和限额', TRUE, 12
);
