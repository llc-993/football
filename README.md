# Football 项目

一个基于 Rust + Actix Web + rbatis + MySQL + Redis 的微服务项目。

## 项目结构

```
football/
├── business/          # 业务服务 (端口: 8080)
│   ├── src/
│   │   ├── main.rs       # 服务入口
│   │   ├── handlers.rs   # 请求处理器
│   │   └── routes.rs     # 路由配置
│   └── Cargo.toml
├── manage/            # 管理服务 (端口: 8081)
│   ├── src/
│   │   ├── main.rs       # 服务入口
│   │   ├── handlers.rs   # 请求处理器
│   │   └── routes.rs     # 路由配置
│   └── Cargo.toml
├── common/            # 公共库
│   ├── src/
│   │   ├── error.rs      # 错误定义
│   │   ├── config.rs     # 配置管理
│   │   └── utils.rs      # 工具函数
│   └── Cargo.toml
├── orm/               # 数据库ORM层
│   ├── src/
│   │   ├── db.rs         # MySQL 连接管理
│   │   ├── cache.rs      # Redis 缓存管理
│   │   ├── entities.rs   # 实体定义
│   │   └── repositories.rs # 仓储层
│   └── Cargo.toml
├── migrations/        # 数据库迁移文件
├── config.toml        # 配置文件
├── docker-compose.yml # Docker 编排文件
└── Cargo.toml         # Workspace 配置
```

## 技术栈

- **Web框架**: Actix Web 4.9
- **异步运行时**: Tokio
- **ORM**: rbatis 4.5
- **数据库**: MySQL 8.0
- **缓存**: Redis 7
- **序列化**: Serde
- **日志**: log + env_logger
- **配置**: config + dotenv

## 快速开始

### 前置条件

- Rust 1.70+
- MySQL 8.0+
- Redis 7+
- Docker & Docker Compose (可选)

### 1. 使用 Docker 启动依赖服务

```bash
# 启动 MySQL 和 Redis
docker-compose up -d

# 查看服务状态
docker-compose ps

# 查看日志
docker-compose logs -f
```

### 2. 配置环境变量

```bash
# 复制环境变量示例文件
cp env.example .env

# 编辑 .env 文件，修改配置
vim .env
```

### 3. 初始化数据库

```bash
# 连接到 MySQL
mysql -h 127.0.0.1 -u root -p

# 执行迁移脚本
source migrations/001_create_users_table.sql
```

### 4. 构建项目

```bash
# 构建所有服务
cargo build

# 或者构建特定服务
cargo build --bin football-business
cargo build --bin football-manage
```

### 5. 运行服务

#### 运行 Business 服务 (端口 8080)

```bash
# 设置日志级别
export RUST_LOG=info

# 运行服务
cargo run --bin football-business
```

访问: http://localhost:8080

#### 运行 Manage 服务 (端口 8081)

```bash
# 在新终端运行
export RUST_LOG=info
export SERVER_PORT=8081

cargo run --bin football-manage
```

访问: http://localhost:8081

### 6. 测试 API

#### Business 服务 API

```bash
# 服务信息
curl http://localhost:8080/

# 健康检查
curl http://localhost:8080/health

# 设置缓存
curl -X POST http://localhost:8080/api/cache/set \
  -H "Content-Type: application/json" \
  -d '{"key": "test_key", "value": "test_value"}'

# 获取缓存
curl http://localhost:8080/api/cache/get/test_key
```

#### Manage 服务 API

```bash
# 服务信息
curl http://localhost:8081/

# 健康检查
curl http://localhost:8081/health

# 获取所有用户
curl http://localhost:8081/api/users

# 根据ID获取用户
curl http://localhost:8081/api/users/1

# 根据用户名获取用户
curl http://localhost:8081/api/users/username/admin
```

## 开发指南

### 添加新的依赖

项目使用 Workspace 统一管理依赖，在根目录的 `Cargo.toml` 中添加：

```toml
[workspace.dependencies]
your-dependency = "version"
```

然后在子 crate 的 `Cargo.toml` 中引用：

```toml
[dependencies]
your-dependency = { workspace = true }
```

### 添加新的数据库实体

1. 在 `orm/src/entities.rs` 中定义实体：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YourEntity {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
```

2. 在 `orm/src/repositories.rs` 中实现仓储：

```rust
pub struct YourRepository;

impl YourRepository {
    pub async fn find_by_name(&self, name: &str) -> AppResult<Option<YourEntity>> {
        let rb = get_db();
        let sql = "SELECT * FROM your_table WHERE name = ? LIMIT 1";
        let result = rb.query_decode(sql, vec![name.into()]).await?;
        Ok(result)
    }
}
```

### 添加新的 API 端点

1. 在 `handlers.rs` 中添加处理器函数：

```rust
pub async fn your_handler() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success("OK"))
}
```

2. 在 `routes.rs` 中注册路由：

```rust
cfg.route("/your-path", web::get().to(handlers::your_handler));
```

## 配置说明

### 环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| SERVER_HOST | 服务器地址 | 0.0.0.0 |
| SERVER_PORT | 服务器端口 | 8080 |
| DATABASE_URL | MySQL 连接字符串 | mysql://root:password@localhost:3306/football |
| DATABASE_MAX_CONNECTIONS | 数据库最大连接数 | 10 |
| REDIS_URL | Redis 连接字符串 | redis://localhost:6379 |
| REDIS_POOL_SIZE | Redis 连接池大小 | 10 |
| LOG_LEVEL | 日志级别 | info |
| RUN_MODE | 运行模式 | development |

### 配置文件

- `config.toml`: 开发环境配置
- `config.production.toml`: 生产环境配置

使用 `RUN_MODE` 环境变量切换配置：

```bash
export RUN_MODE=production
cargo run
```

## 代码检查和测试

### 格式化代码

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

### 运行测试

```bash
cargo test
```

## Docker 部署

### 构建镜像

```bash
# Business 服务
docker build -t football-business:latest -f business/Dockerfile .

# Manage 服务
docker build -t football-manage:latest -f manage/Dockerfile .
```

### 运行容器

```bash
docker-compose up -d
```

## 目录说明

- **business/**: 业务服务，提供核心业务 API
- **manage/**: 管理服务，提供后台管理 API
- **common/**: 公共库，包含错误处理、配置管理、工具函数
- **orm/**: ORM 层，包含数据库和缓存操作
- **migrations/**: 数据库迁移脚本

## API 文档

### Business API

- `GET /` - 服务信息
- `GET /health` - 健康检查
- `POST /api/cache/set` - 设置缓存
- `GET /api/cache/get/{key}` - 获取缓存

### Manage API

- `GET /` - 服务信息
- `GET /health` - 健康检查
- `GET /api/users` - 获取所有用户
- `GET /api/users/{id}` - 根据ID获取用户
- `GET /api/users/username/{username}` - 根据用户名获取用户

## 常见问题

### 1. 数据库连接失败

检查 MySQL 是否启动：

```bash
docker-compose ps mysql
```

查看 MySQL 日志：

```bash
docker-compose logs mysql
```

### 2. Redis 连接失败

检查 Redis 是否启动：

```bash
docker-compose ps redis
```

### 3. 编译错误

清理并重新构建：

```bash
cargo clean
cargo build
```

## 许可证

MIT OR Apache-2.0
