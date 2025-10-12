# 🚀 API 多语言响应使用指南（极简版）

## 🎯 设计目标

实现**最简洁**的多语言 API 响应，让开发者**零负担**：
- ✅ **无需传递任何参数**：不需要 `req`、`i18n`、`lang` 等
- ✅ **一行代码搞定**：`ApiResponse::success_i18n(data, key)`
- ✅ **自动语言检测**：从 `Accept-Language` 请求头自动识别
- ✅ **全局翻译系统**：基于 Thread-local 的零成本抽象

---

## 📂 项目结构

```
business/
├── src/
│   ├── conf/                      # 配置模块
│   │   ├── mod.rs                # 模块导出
│   │   ├── i18n.rs               # Fluent 多语言核心 + Thread-local
│   │   ├── middleware.rs         # 语言检测中间件（自动设置 thread-local）
│   │   ├── response_ext.rs       # 响应扩展（已废弃）
│   │   └── response_helper.rs    # 响应辅助（已废弃）
│   ├── handlers.rs               # 业务处理器
│   ├── routes.rs                 # 路由配置
│   └── main.rs                   # 启动入口
├── locales/                      # 多语言资源文件
│   ├── zh-CN.ftl                # 中文简体
│   └── en-US.ftl                # 英文
└── API_USAGE.md                  # 本文档

common/
└── src/
    └── common/
        └── response.rs           # ApiResponse（支持 i18n）
```

---

## 🌟 核心特性

### 1. **极简 API** - 无参数设计

**之前的方式**（需要传参）：
```rust
pub async fn handler(req: HttpRequest, i18n: web::Data<I18nBundles>) -> impl Responder {
    i18n_resp!(req, i18n).success(data, key)
}
```

**现在的方式**（无需传参）：
```rust
pub async fn handler() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success_i18n(data, keys::SUCCESS))
}
```

### 2. **Thread-local 自动上下文**

- 中间件自动从请求头解析语言
- 将语言设置到当前线程的 thread-local 变量
- `ApiResponse` 自动从 thread-local 读取语言并翻译
- **零运行时开销**

### 3. **全局翻译函数**

```rust
use crate::conf::i18n;

// 获取当前语言
let lang = i18n::get_current_language();

// 翻译消息
let msg = i18n::translate(keys::SUCCESS);
```

---

## 📖 API 使用示例

### 1. 成功响应（带数据）

```rust
use actix_web::{HttpResponse, Responder};
use football_common::ApiResponse;
use crate::conf::keys;

pub async fn get_user() -> impl Responder {
    let user = User {
        id: 1,
        name: "张三".to_string(),
    };
    
    // 一行搞定！自动根据请求语言翻译
    HttpResponse::Ok().json(ApiResponse::success_i18n(user, keys::USER_FOUND))
}
```

**响应示例（中文）**：
```json
{
  "data": {
    "id": 1,
    "name": "张三"
  },
  "msg": "用户查询成功",
  "code": 200
}
```

**响应示例（英文）**：
```json
{
  "data": {
    "id": 1,
    "name": "张三"
  },
  "msg": "User found successfully",
  "code": 200
}
```

---

### 2. 成功响应（无数据）

```rust
pub async fn delete_user() -> impl Responder {
    // 执行删除逻辑...
    
    // 返回无数据成功响应
    HttpResponse::Ok().json(ApiResponse::<()>::ok_i18n(keys::USER_DELETED))
}
```

**响应示例（中文）**：
```json
{
  "data": null,
  "msg": "用户删除成功",
  "code": 200
}
```

**响应示例（英文）**：
```json
{
  "data": null,
  "msg": "User deleted successfully",
  "code": 200
}
```

---

### 3. 错误响应

```rust
pub async fn get_user_by_id(id: i32) -> impl Responder {
    // 查询用户...
    if user.is_none() {
        // 返回 404 错误
        return HttpResponse::NotFound()
            .json(ApiResponse::not_found_i18n(keys::USER_NOT_FOUND));
    }
    
    // 返回成功
    HttpResponse::Ok().json(ApiResponse::success_i18n(user, keys::SUCCESS))
}
```

**错误响应示例（中文）**：
```json
{
  "data": null,
  "msg": "用户不存在",
  "code": 404
}
```

**错误响应示例（英文）**：
```json
{
  "data": null,
  "msg": "User not found",
  "code": 404
}
```

---

### 4. 其他错误类型

```rust
// 400 Bad Request
HttpResponse::BadRequest()
    .json(ApiResponse::bad_request_i18n(keys::VALIDATION_ERROR))

// 401 Unauthorized
HttpResponse::Unauthorized()
    .json(ApiResponse::unauthorized_i18n(keys::UNAUTHORIZED))

// 403 Forbidden
HttpResponse::Forbidden()
    .json(ApiResponse::forbidden_i18n(keys::FORBIDDEN))

// 500 Internal Server Error
HttpResponse::InternalServerError()
    .json(ApiResponse::error_i18n(keys::INTERNAL_ERROR))

// 自定义状态码
HttpResponse::ServiceUnavailable()
    .json(ApiResponse::error_i18n_code(keys::DB_ERROR, 503))
```

---

## 🔑 完整的 API 方法列表

### 成功响应

| 方法 | 说明 | 示例 |
|-----|------|------|
| `success_i18n(data, key)` | 带数据的成功响应 | `ApiResponse::success_i18n(user, keys::SUCCESS)` |
| `ok_i18n(key)` | 无数据的成功响应 | `ApiResponse::<()>::ok_i18n(keys::USER_DELETED)` |

### 错误响应

| 方法 | 状态码 | 说明 |
|-----|--------|------|
| `error_i18n(key)` | 500 | 通用错误 |
| `error_i18n_code(key, code)` | 自定义 | 自定义状态码错误 |
| `bad_request_i18n(key)` | 400 | 参数错误 |
| `unauthorized_i18n(key)` | 401 | 未授权 |
| `forbidden_i18n(key)` | 403 | 禁止访问 |
| `not_found_i18n(key)` | 404 | 资源未找到 |

---

## 🔑 消息键常量（Keys）

所有消息键定义在 `conf::keys` 模块：

```rust
use crate::conf::keys;

// 通用消息
keys::SUCCESS              // 操作成功
keys::FAIL                 // 操作失败
keys::INTERNAL_ERROR       // 内部错误

// 用户相关
keys::USER_NOT_FOUND       // 用户不存在
keys::USER_CREATED         // 用户创建成功
keys::USER_UPDATED         // 用户更新成功
keys::USER_DELETED         // 用户删除成功
keys::USER_FOUND           // 用户查询成功
keys::USER_ALREADY_EXISTS  // 用户已存在
keys::USER_WELCOME         // 欢迎消息（支持参数）

// 认证相关
keys::UNAUTHORIZED         // 未授权
keys::FORBIDDEN            // 禁止访问
keys::INVALID_TOKEN        // 无效令牌
keys::LOGIN_SUCCESS        // 登录成功
keys::LOGIN_FAILED         // 登录失败

// 验证相关
keys::VALIDATION_ERROR     // 参数验证失败
keys::INVALID_PARAM        // 无效参数
keys::REQUIRED_FIELD       // 必填字段

// 数据库相关
keys::DB_ERROR             // 数据库操作失败
keys::DB_CONNECTION_FAILED // 数据库连接失败

// Redis 相关
keys::REDIS_ERROR          // 缓存操作失败
keys::REDIS_CONNECTION_FAILED // Redis连接失败
keys::CACHE_MISS           // 缓存未命中

// 操作相关
keys::OPERATION_SUCCESS    // 操作成功
keys::OPERATION_FAILED     // 操作失败
keys::ITEMS_COUNT          // 项目数量（支持复数）
```

---

## 🌍 语言检测

语言由中间件自动检测，从 `Accept-Language` 请求头提取：

```bash
# 中文请求
curl -H "Accept-Language: zh-CN" http://localhost:8080/api/users

# 英文请求
curl -H "Accept-Language: en-US" http://localhost:8080/api/users
```

**支持的语言格式**：
- 中文：`zh-CN`, `zh-Hans`, `zh`, `chinese`
- 英文：`en-US`, `en-GB`, `en`, `english`

**默认语言**：如果请求头中没有语言信息，默认使用**中文简体**。

---

## ⚙️ 技术实现原理

### 1. **Thread-local 存储**

```rust
// 在 conf/i18n.rs 中
thread_local! {
    static CURRENT_LANGUAGE: RefCell<Language> = RefCell::new(Language::ZhCN);
}

pub fn set_current_language(lang: Language) {
    CURRENT_LANGUAGE.with(|current| {
        *current.borrow_mut() = lang;
    });
}

pub fn get_current_language() -> Language {
    CURRENT_LANGUAGE.with(|current| *current.borrow())
}
```

### 2. **中间件自动设置语言**

```rust
// 在 conf/middleware.rs 中
fn call(&self, req: ServiceRequest) -> Self::Future {
    let lang = req
        .headers()
        .get("Accept-Language")
        .and_then(|h| h.to_str().ok())
        .map(Language::from_accept_language)
        .unwrap_or(Language::ZhCN);

    // 设置当前线程的语言
    super::i18n::set_current_language(lang);
    
    // 继续处理请求...
}
```

### 3. **ApiResponse 自动翻译**

```rust
// 在 common/src/common/response.rs 中
fn translate_key(key: &str) -> String {
    if let Some(translator) = I18N_TRANSLATOR.get() {
        translator(key)  // 调用注册的翻译器
    } else {
        key.to_string()  // 未初始化时返回 key
    }
}

impl<T> ApiResponse<T> {
    pub fn success_i18n(data: T, message_key: &str) -> Self {
        Self {
            data: Some(data),
            message: translate_key(message_key),  // 自动翻译
            code: http_status::SUCCESS,
        }
    }
}
```

### 4. **全局翻译器注册**

```rust
// 在 business/src/main.rs 中
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ...
    
    // 注册全局 i18n 翻译器到 common 模块
    football_common::init_i18n_translator(i18n::translate);
    
    // ...
}
```

---

## 📝 完整示例

### 用户管理 API

```rust
use actix_web::{web, HttpResponse, Responder};
use football_common::ApiResponse;
use crate::conf::keys;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

/// 获取用户列表
pub async fn list_users() -> impl Responder {
    let users = vec![
        User { id: 1, name: "张三".to_string(), email: "zhang@example.com".to_string() },
        User { id: 2, name: "李四".to_string(), email: "li@example.com".to_string() },
    ];
    
    HttpResponse::Ok().json(ApiResponse::success_i18n(users, keys::SUCCESS))
}

/// 获取单个用户
pub async fn get_user(id: web::Path<i32>) -> impl Responder {
    // 模拟查询
    if *id == 0 {
        return HttpResponse::NotFound()
            .json(ApiResponse::not_found_i18n(keys::USER_NOT_FOUND));
    }
    
    let user = User {
        id: *id,
        name: "张三".to_string(),
        email: "zhang@example.com".to_string(),
    };
    
    HttpResponse::Ok().json(ApiResponse::success_i18n(user, keys::USER_FOUND))
}

/// 创建用户
pub async fn create_user(user: web::Json<User>) -> impl Responder {
    // 模拟创建逻辑
    HttpResponse::Ok().json(ApiResponse::success_i18n(user.into_inner(), keys::USER_CREATED))
}

/// 更新用户
pub async fn update_user(id: web::Path<i32>, user: web::Json<User>) -> impl Responder {
    // 模拟更新逻辑
    HttpResponse::Ok().json(ApiResponse::success_i18n(user.into_inner(), keys::USER_UPDATED))
}

/// 删除用户
pub async fn delete_user(id: web::Path<i32>) -> impl Responder {
    // 模拟删除逻辑
    HttpResponse::Ok().json(ApiResponse::<()>::ok_i18n(keys::USER_DELETED))
}
```

---

## 🎨 添加新的多语言消息

### 步骤 1：在 FTL 文件中添加翻译

**`locales/zh-CN.ftl`**：
```ftl
order-created = 订单创建成功
order-not-found = 订单不存在
order-status = 订单状态：{$status}
```

**`locales/en-US.ftl`**：
```ftl
order-created = Order created successfully
order-not-found = Order not found
order-status = Order status: {$status}
```

### 步骤 2：在 keys 模块中添加常量

**`conf/i18n.rs`**：
```rust
pub mod keys {
    // ... 现有的 keys ...
    
    // 订单相关
    pub const ORDER_CREATED: &str = "order-created";
    pub const ORDER_NOT_FOUND: &str = "order-not-found";
    pub const ORDER_STATUS: &str = "order-status";
}
```

### 步骤 3：在代码中使用

```rust
pub async fn create_order() -> impl Responder {
    // 创建订单逻辑...
    
    HttpResponse::Ok().json(ApiResponse::success_i18n(order, keys::ORDER_CREATED))
}

pub async fn get_order_status() -> impl Responder {
    // 如果需要参数，可以使用 translate_with_args
    use crate::conf::i18n;
    let msg = i18n::translate(keys::ORDER_STATUS);  // 或使用 translate_with_args
    
    HttpResponse::Ok().json(ApiResponse::success_msg(status, msg))
}
```

---

## ✨ 优势对比

| 特性 | 之前（v1） | 之前（v2） | **现在（v3）** |
|-----|-----------|-----------|---------------|
| **参数传递** | `req` + `i18n` | `i18n: I18nCtx` | **无需任何参数** ✅ |
| **代码行数** | 3-5 行 | 1 行 | **1 行** ✅ |
| **上下文管理** | 手动提取 | 提取器 | **Thread-local 自动** ✅ |
| **学习成本** | 高 | 中 | **极低** ✅ |
| **性能开销** | 低 | 低 | **零开销** ✅ |
| **代码侵入性** | 高 | 中 | **极低** ✅ |

---

## 🚀 快速开始

### 1. 启动服务

```bash
cd /Users/m1pro/rustproject/football
cargo run --bin football-business
```

### 2. 测试多语言

```bash
# 中文测试
curl -H "Accept-Language: zh-CN" http://localhost:8080/health
# 输出: {"data":"OK","msg":"操作成功","code":200}

# 英文测试
curl -H "Accept-Language: en-US" http://localhost:8080/health
# 输出: {"data":"OK","msg":"Success","code":200}

# 测试错误响应
curl -H "Accept-Language: zh-CN" http://localhost:8080/api/test/error
# 输出: {"data":null,"msg":"用户不存在","code":404}

curl -H "Accept-Language: en-US" http://localhost:8080/api/test/error
# 输出: {"data":null,"msg":"User not found","code":404}
```

---

## 🎯 最佳实践

### ✅ 推荐做法

1. **使用 `_i18n` 后缀方法**：所有需要多语言的地方都使用 `success_i18n`、`error_i18n` 等方法
2. **统一使用 keys 常量**：避免硬编码字符串，所有消息键都定义在 `keys` 模块
3. **在 FTL 文件中管理翻译**：便于维护和扩展，支持专业翻译人员协作
4. **保持翻译文件同步**：确保所有语言版本都有对应的翻译
5. **利用 Fluent 特性**：使用参数化消息、复数形式等高级功能

### ⚠️ 避免做法

1. ❌ 不要在代码中硬编码消息字符串
2. ❌ 不要混用 i18n 和非 i18n 方法
3. ❌ 不要忘记添加新的消息键到 FTL 文件
4. ❌ 不要在 handler 中手动管理语言上下文

---

## 📊 性能说明

### Thread-local 零开销

- **无锁设计**：thread-local 变量无需锁，读取开销几乎为零
- **无堆分配**：语言枚举是栈上的 Copy 类型
- **编译期优化**：Rust 编译器会优化 thread-local 访问

### Fluent 资源预加载

- **启动时加载**：所有 FTL 文件在启动时解析并缓存
- **线程安全共享**：使用 `Arc<FluentResource>` 在线程间共享
- **按需创建 Bundle**：每次翻译时创建临时 Bundle，避免锁竞争

---

## 📖 总结

通过结合 **Thread-local 存储** + **中间件自动注入** + **全局翻译器注册**，我们实现了：

✨ **极简 API**：`ApiResponse::success_i18n(data, key)` 一行搞定  
🚀 **零参数**：无需传递任何上下文参数  
⚡ **零开销**：Thread-local 的零成本抽象  
🌍 **全自动**：语言检测、设置、翻译全自动  
🎯 **低侵入**：业务代码几乎感知不到 i18n 的存在  

这是目前 **最优雅、最简洁** 的 Rust Web 多语言解决方案！🎉
