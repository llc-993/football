# 通用消息
common-success = 操作成功
common-fail = 操作失败
common-internal-error = 内部服务器错误

# 用户相关
user-not-found = 用户不存在
user-created = 用户创建成功
user-updated = 用户更新成功
user-deleted = 用户删除成功
user-already-exists = 用户已存在
user-found = 找到 {$count} 个用户
user-welcome = 欢迎，{$username}！

# 认证相关
auth-unauthorized = 未授权访问
auth-forbidden = 禁止访问
auth-invalid-token = 无效的令牌
auth-login-success = 登录成功
auth-login-failed = 登录失败，用户名或密码错误

# 验证相关
validation-error = 参数验证错误
validation-invalid-param = 无效的参数：{$param}
validation-required-field = 必填字段缺失：{$field}

# 数据库相关
db-error = 数据库错误
db-connection-failed = 数据库连接失败

# Redis 相关
redis-error = Redis错误
redis-connection-failed = Redis连接失败

# 缓存相关
cache-miss = 缓存未命中

# 操作相关（带参数示例）
operation-success = {$operation}成功
operation-failed = {$operation}失败

# 复数支持示例
items-count = 
    { $count ->
        [0] 没有项目
        [1] 1个项目
       *[other] {$count}个项目
    }

