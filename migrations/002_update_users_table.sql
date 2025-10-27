-- 更新用户表，添加账号类型和用户等级字段
ALTER TABLE users 
ADD COLUMN account_type TINYINT NOT NULL DEFAULT 1 COMMENT '账号类型: 1-普通用户, 2-代理账号, 3-测试账号',
ADD COLUMN user_level INT NOT NULL DEFAULT 1 COMMENT '用户等级';

-- 为新增字段添加索引
ALTER TABLE users 
ADD INDEX idx_account_type (account_type),
ADD INDEX idx_user_level (user_level);

-- 更新现有数据的默认值
UPDATE users SET account_type = 1, user_level = 1 WHERE account_type IS NULL OR user_level IS NULL;
