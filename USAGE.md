# Football 项目使用说明

## 快速开始

### 1. 启动依赖服务

```bash
# 启动 MySQL 和 Redis
docker-compose up -d

# 等待服务就绪
docker-compose ps
```

### 2. 配置环境变量

```bash
# 复制环境变量示例文件并修改
cp env.example .env

# 编辑 .env 文件
# 默认配置：
# - MySQL: mysql://root:password@localhost:3306/football
# - Redis: redis://localhost:6379
# - 端口: 8080 (business), 8081 (manage)
```

### 3. 初始化数据库

```bash
# 连接到 MySQL
mysql -h 127.0.0.1 -u root -ppassword

# 创建数据库（如果不存在）
CREATE DATABASE IF NOT EXISTS football CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

# 执行迁移脚本
USE football;
SOURCE migrations/001_create_users_table.sql;
```

### 4. 运行服务

#### Business 服务 (端口 8080)

```bash
export RUST_LOG=info
cargo run --bin football-business
```

#### Manage 服务 (端口 8081)

```bash
# 在新终端中运行
export RUST_LOG=info
export SERVER_PORT=8081
cargo run --bin football-manage
```

## API 测试

### Business 服务 API (端口 8080)

```bash
# 1. 服务信息
curl http://localhost:8080/

# 响应示例：
# {
#   "success": true,
#   "data": {
#     "name": "Football Business API",
#     "version": "0.1.0",
#     "status": "running"
#   },
#   "message": "操作成功"
# }

# 2. 健康检查
curl http://localhost:8080/health

# 3. 设置缓存
curl -X POST http://localhost:8080/api/cache/set \
  -H "Content-Type: application/json" \
  -d '{
    "key": "user:1001",
    "value": "{\"username\":\"test\",\"email\":\"test@example.com\"}"
  }'

# 4. 获取缓存
curl http://localhost:8080/api/cache/get/user:1001
```

### Manage 服务 API (端口 8081)

```bash
# 1. 服务信息
curl http://localhost:8081/

# 2. 健康检查
curl http://localhost:8081/health

# 3. 获取所有用户
curl http://localhost:8081/api/users

# 4. 根据ID获取用户
curl http://localhost:8081/api/users/1

# 5. 根据用户名获取用户
curl http://localhost:8081/api/users/username/admin
```

## 插入测试数据

```sql
-- 连接到 MySQL
mysql -h 127.0.0.1 -u root -ppassword football

-- 插入测试用户
INSERT INTO users (username, email, password_hash, created_at, updated_at) VALUES
('admin', 'admin@example.com', '$2a$12$hashvalue1', NOW(), NOW()),
('user1', 'user1@example.com', '$2a$12$hashvalue2', NOW(), NOW()),
('user2', 'user2@example.com', '$2a$12$hashvalue3', NOW(), NOW());

-- 查询用户
SELECT * FROM users;
```

## 项目结构说明

```
football/
├── business/          # 业务服务 (8080)
│   ├── handlers.rs   # API 处理器（缓存操作等）
│   └── routes.rs     # 路由配置
├── manage/            # 管理服务 (8081)
│   ├── handlers.rs   # API 处理器（用户管理等）
│   └── routes.rs     # 路由配置
├── common/            # 公共库
│   ├── error.rs      # 统一错误处理
│   ├── config.rs     # 配置管理
│   └── utils.rs      # 工具函数
└── orm/               # ORM 层
    ├── db.rs         # MySQL 连接
    ├── cache.rs      # Redis 缓存
    ├── entities.rs   # 数据实体
    └── repositories.rs # 数据仓储
```

## 开发提示

### 添加新的 API 端点

1. 在 `handlers.rs` 中添加处理函数
2. 在 `routes.rs` 中注册路由
3. 重启服务测试

### 添加新的数据表

1. 在 `migrations/` 创建 SQL 迁移文件
2. 在 `orm/src/entities.rs` 定义实体结构
3. 在 `orm/src/repositories.rs` 实现仓储方法
4. 在服务中使用新的仓储

### 环境变量配置

| 变量 | 说明 | 默认值 |
|------|------|--------|
| SERVER_HOST | 服务器监听地址 | 0.0.0.0 |
| SERVER_PORT | 服务器端口 | 8080 |
| DATABASE_URL | MySQL连接字符串 | mysql://root:password@localhost:3306/football |
| REDIS_URL | Redis连接字符串 | redis://localhost:6379 |
| RUST_LOG | 日志级别 | info |

## 常见问题

### 数据库连接失败

```bash
# 检查 MySQL 是否运行
docker-compose ps mysql

# 查看 MySQL 日志
docker-compose logs mysql

# 手动测试连接
mysql -h 127.0.0.1 -u root -ppassword
```

### Redis 连接失败

```bash
# 检查 Redis 是否运行
docker-compose ps redis

# 测试 Redis 连接
redis-cli ping
```

### 端口被占用

```bash
# 修改端口
export SERVER_PORT=9090
cargo run --bin football-business
```

## 生产部署

### 使用 Release 模式构建

```bash
cargo build --release

# 运行
./target/release/football-business
./target/release/football-manage
```

### 使用 Docker

```bash
# 构建镜像
docker build -t football-business:latest .

# 运行
docker run -d \
  -p 8080:8080 \
  -e DATABASE_URL=mysql://root:password@mysql:3306/football \
  -e REDIS_URL=redis://redis:6379 \
  football-business:latest
```

## 性能调优

### 数据库连接池

在 `.env` 中调整：
```
DATABASE_MAX_CONNECTIONS=20
```

### Redis 连接池

在 `.env` 中调整：
```
REDIS_POOL_SIZE=20
```

### 日志级别

生产环境建议使用 `warn` 或 `error`：
```
RUST_LOG=warn
```

