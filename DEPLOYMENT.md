# 📦 项目部署指南

## 🎯 编译时嵌入资源

本项目已将以下资源在**编译时**嵌入到二进制文件中：

### ✅ 已嵌入的资源

1. **多语言配置文件**（Fluent FTL）
   - `business/locales/zh-CN.ftl` - 中文简体
   - `business/locales/en-US.ftl` - 英文

2. **应用配置文件**（TOML）
   - `business/config.toml` - 开发环境配置
   - `business/config.production.toml` - 生产环境配置

### 实现方式

使用 Rust 内置的 `include_str!` 宏：

```rust
// 多语言文件嵌入（business/src/conf/i18n.rs）
let ftl_string = match lang {
    Language::ZhCN => include_str!("../../locales/zh-CN.ftl"),
    Language::En => include_str!("../../locales/en-US.ftl"),
};

// 配置文件嵌入（business/src/main.rs）
const DEFAULT_CONFIG: &str = include_str!("../config.toml");
const PROD_CONFIG: &str = include_str!("../config.production.toml");
```

---

## 🚀 编译项目

### Debug 模式（开发）

```bash
cd /Users/m1pro/rustproject/football
cargo build --bin football-business
```

生成位置：`target/debug/football-business`

### Release 模式（生产）

```bash
cd /Users/m1pro/rustproject/football
cargo build --release --bin football-business
```

生成位置：`target/release/football-business`（约 11MB）

---

## 📦 部署方式

### 方式 1：独立部署（使用嵌入配置）

```bash
# 1. 编译
cargo build --release --bin football-business

# 2. 复制二进制文件到服务器
scp target/release/football-business user@server:/opt/football/

# 3. 在服务器上直接运行（使用嵌入的配置）
cd /opt/football
./football-business
```

**优势**：
- ✅ 只需要单个二进制文件
- ✅ 无需复制配置文件
- ✅ 配置已嵌入，不会丢失

---

### 方式 2：使用外部配置覆盖

如果需要在服务器上自定义配置：

```bash
# 1. 复制二进制文件
scp target/release/football-business user@server:/opt/football/

# 2. 创建自定义配置文件
cat > /opt/football/business/config.toml <<EOF
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "mysql://root:password@db-server:3306/football"
max_connections = 20

[redis]
url = "redis://:password@redis-server:6379/0"
pool_size = 20

[log]
level = "info"
EOF

# 3. 运行（会优先使用外部配置）
cd /opt/football
./football-business
```

**配置加载优先级**（从高到低）：
1. **环境变量**（`APP_SERVER__HOST=0.0.0.0`）
2. **外部配置文件**（`business/config.toml`）
3. **嵌入的配置**（编译时打包的）

---

### 方式 3：使用环境变量

```bash
# 通过环境变量覆盖配置
export APP_SERVER__HOST=0.0.0.0
export APP_SERVER__PORT=8080
export APP_DATABASE__URL="mysql://root:password@localhost:3306/football"
export RUN_MODE=production  # 使用生产环境配置

./football-business
```

---

## 🌍 生产环境部署

### 设置生产模式

```bash
# 设置环境变量
export RUN_MODE=production

# 运行（会使用嵌入的 config.production.toml）
./football-business
```

### 使用 systemd 服务

创建 `/etc/systemd/system/football-business.service`：

```ini
[Unit]
Description=Football Business API Service
After=network.target mysql.service redis.service

[Service]
Type=simple
User=football
WorkingDirectory=/opt/football
Environment="RUN_MODE=production"
Environment="RUST_LOG=info"
ExecStart=/opt/football/football-business
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

启动服务：

```bash
sudo systemctl daemon-reload
sudo systemctl enable football-business
sudo systemctl start football-business
sudo systemctl status football-business
```

---

## 🔍 验证部署

### 1. 检查服务状态

```bash
# 检查端口
netstat -tuln | grep 8080

# 或使用 lsof
lsof -i:8080
```

### 2. 测试 API

```bash
# 健康检查
curl http://localhost:8080/health

# 中文测试
curl -H "Accept-Language: zh-CN" http://localhost:8080/health
# 输出: {"data":"OK","msg":"操作成功","code":200}

# 英文测试
curl -H "Accept-Language: en-US" http://localhost:8080/health
# 输出: {"data":"OK","msg":"Success","code":200}
```

### 3. 查看日志

```bash
# 如果使用 systemd
journalctl -u football-business -f

# 或直接运行时查看
RUST_LOG=info ./football-business
```

---

## 📊 二进制文件信息

```bash
$ ls -lh target/release/football-business
-rwxr-xr-x  1 user  staff  11M Oct 12 11:49 football-business

$ file target/release/football-business
football-business: Mach-O 64-bit executable arm64
```

**包含内容**：
- ✅ 应用代码
- ✅ 所有依赖库（Actix Web、Rbatis、Redis 等）
- ✅ 多语言文件（`zh-CN.ftl`、`en-US.ftl`）
- ✅ 配置文件（`config.toml`、`config.production.toml`）
- ✅ Sa-Token 认证框架

---

## 🎯 优势

### 独立部署
- 📦 **单文件部署**：只需要一个可执行文件
- 🚀 **快速部署**：无需配置额外的配置文件路径
- 🔒 **配置安全**：默认配置嵌入，不易丢失

### 灵活配置
- 🔧 **可覆盖**：支持外部配置文件覆盖
- 🌍 **环境变量**：支持通过环境变量动态配置
- 🎚️ **多环境**：自动根据 `RUN_MODE` 切换配置

### 性能优化
- ⚡ **零 I/O**：配置和多语言文件直接从内存读取
- 🏃 **快速启动**：无需读取文件系统
- 💾 **内存友好**：配置在编译时已优化

---

## 📝 总结

现在你的 `football-business` 服务：
- ✅ **多语言文件已嵌入**：支持中文/英文自动切换
- ✅ **配置文件已嵌入**：包含开发和生产环境配置
- ✅ **独立部署**：单个 11MB 的可执行文件
- ✅ **灵活配置**：支持外部配置和环境变量覆盖

部署时只需要复制一个二进制文件，即可运行完整的服务！🎉

