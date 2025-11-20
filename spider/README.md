# Football Spider

独立的爬虫项目，可以抓取网页数据，拦截HTTP请求和监听WebSocket消息。

## 功能特性

- ✅ HTTP请求（支持代理）
- ✅ **HTTP请求/响应拦截和打印**
- ✅ **WebSocket连接和消息监听**
- ✅ HTML解析
- ✅ API端点自动发现
- ✅ 错误重试机制
- ✅ 可配置的请求间隔
- ✅ 详细的日志记录

## 使用方法

### 1. 运行爬虫

```bash
# 在项目根目录
cargo run -p football-spider

# 或者进入spider目录
cd spider
cargo run
```

### 2. 爬取特定网站

当前配置为爬取 `https://sportbet.one/zh-CN/sports`，程序会：
- 📤 打印所有HTTP请求信息（URL、Headers等）
- 📥 打印所有HTTP响应信息（Status、Headers、Body）
- 🔍 自动发现页面中的API接口并请求
- 🔌 自动发现并连接WebSocket，监听消息
- 📨 实时打印WebSocket接收到的数据

### 3. 配置

编辑 `config.toml` 文件来配置爬虫参数：

```toml
user_agent = "Mozilla/5.0..."
timeout = 30
delay_ms = 1000
max_retries = 3
use_proxy = false
```

### 4. 环境变量

可以通过环境变量指定配置文件路径：

```bash
SPIDER_CONFIG=./config.toml cargo run -p football-spider
```

## 项目结构

```
spider/
├── Cargo.toml          # 项目配置
├── config.toml         # 爬虫配置
├── README.md           # 说明文档
└── src/
    ├── main.rs         # 主入口
    ├── config.rs       # 配置管理
    ├── error.rs        # 错误定义
    └── spider.rs       # 爬虫核心逻辑
```

## 扩展开发

### 添加新的爬虫任务

在 `spider.rs` 的 `run` 方法中添加你的爬虫逻辑：

```rust
pub async fn run(&self) -> Result<()> {
    // 你的爬虫逻辑
    let url = "https://example.com";
    let content = self.fetch_page(url).await?;
    // 解析和处理数据
    Ok(())
}
```

### 解析HTML

使用 `parse_html` 方法解析HTML：

```rust
let results = spider.parse_html(&html, "div.title")?;
```

## 依赖说明

- `reqwest`: HTTP客户端
- `scraper`: HTML解析
- `tokio`: 异步运行时
- `tracing`: 日志记录

