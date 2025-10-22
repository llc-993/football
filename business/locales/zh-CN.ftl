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

# 用户操作成功
user-register-success = 注册成功
user-login-success = 登录成功
user-password-changed = 密码修改成功
user-password-reset = 密码重置成功
user-detail-success = 获取用户信息成功

# 用户错误
user-already-exists = 用户名已存在
email-already-exists = 邮箱已被注册
username-or-password-error = 用户名或密码错误
account-frozen-or-disabled = 账号已被冻结或禁用
old-password-error = 原密码错误
email-not-registered = 邮箱未注册
verification-code-error = 验证码错误或已过期

# 参数校验错误
password-required = 密码不能为空
email-or-phone-required = 邮箱和手机号至少填写一个

# 系统错误
token-generation-failed = Token生成失败
update-login-time-failed = 更新登录时间失败

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

# 钱包相关
wallet-balance = 钱包余额
wallet-recharge-success = 充值成功
wallet-withdraw-success = 提现成功
wallet-insufficient-balance = 余额不足
wallet-frozen = 钱包已冻结
wallet-active = 钱包正常
wallet-transaction-history = 交易记录

