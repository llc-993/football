// 通用 API 响应结构
use serde::Serialize;

// 全局 i18n 翻译函数指针（由业务模块注入）
use std::sync::OnceLock;

static I18N_TRANSLATOR: OnceLock<fn(&str) -> String> = OnceLock::new();

/// 初始化 i18n 翻译器（在业务模块启动时调用）
pub fn init_i18n_translator(translator: fn(&str) -> String) {
    let _ = I18N_TRANSLATOR.set(translator);
}

/// 内部翻译函数
fn translate_key(key: &str) -> String {
    if let Some(translator) = I18N_TRANSLATOR.get() {
        translator(key)
    } else {
        // 如果未初始化翻译器，直接返回 key
        key.to_string()
    }
}

/// HTTP 状态码常量
pub mod http_status {
    pub const SUCCESS: i32 = 200;
    pub const CREATED: i32 = 201;
    pub const BAD_REQUEST: i32 = 400;
    pub const UNAUTHORIZED: i32 = 401;
    pub const FORBIDDEN: i32 = 403;
    pub const NOT_FOUND: i32 = 404;
    pub const INTERNAL_ERROR: i32 = 500;
}

/// 响应消息常量
pub mod constants {
    pub const SUCCESS: &str = "SUCCESS";
    pub const FAIL: &str = "FAIL";
}

/// 统一的 API 响应格式
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    /// 响应数据
    pub data: Option<T>,
    /// 响应消息
    #[serde(rename = "msg")]
    pub message: String,
    /// 状态码
    pub code: i32,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应（只有数据）
    ///
    /// 默认: msg = "操作成功", code = 200
    /// 
    /// # 示例
    /// ```
    /// let response = ApiResponse::success(user_data);
    /// // { data: Some(user_data), msg: "操作成功", code: 200 }
    /// ```
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            message: constants::SUCCESS.to_string(),
            code: http_status::SUCCESS,
        }
    }

    /// 创建成功响应（数据 + 自定义消息）
    ///
    /// 默认: code = 200
    /// 
    /// # 示例
    /// ```
    /// let response = ApiResponse::success_msg(user_data, "用户创建成功");
    /// // { data: Some(user_data), msg: "用户创建成功", code: 200 }
    /// ```
    pub fn success_msg(data: T, message: impl Into<String>) -> Self {
        Self {
            data: Some(data),
            message: message.into(),
            code: http_status::SUCCESS,
        }
    }

    /// 创建成功响应（数据 + 消息 + 状态码）
    /// 
    /// # 示例
    /// ```
    /// let response = ApiResponse::success_full(user_data, "用户创建成功", 201);
    /// // { data: Some(user_data), msg: "用户创建成功", code: 201 }
    /// ```
    pub fn success_full(data: T, message: impl Into<String>, code: i32) -> Self {
        Self {
            data: Some(data),
            message: message.into(),
            code,
        }
    }
    /// 创建成功响应（无数据）
    ///
    /// 等价于 Kotlin: success(null)
    /// 默认: msg = "操作成功", code = 200
    ///
    /// # 示例
    /// ```
    /// let response = ApiResponse::success1();
    /// // { data: None, msg: "操作成功", code: 200 }
    /// ```
    pub fn success1() -> Self {
        Self {
            data: None,
            message: constants::SUCCESS.to_string(),
            code: http_status::SUCCESS,
        }
    }

    /// 创建成功响应（无数据 + 自定义消息）
    ///
    /// # 示例
    /// ```
    /// let response = ApiResponse::ok1("删除成功");
    /// // { data: None, msg: "删除成功", code: 200 }
    /// ```
    pub fn ok1(message: impl Into<String>) -> Self {
        Self {
            data: None,
            message: message.into(),
            code: http_status::SUCCESS,
        }
    }

    // ============= i18n 方法（自动翻译） =============

    /// 创建成功响应（数据 + i18n 消息键）
    /// 
    /// 自动根据当前语言翻译消息
    /// 
    /// # 示例
    /// ```
    /// let response = ApiResponse::success_i18n(user_data, "user-created");
    /// // 中文: { data: Some(user_data), msg: "用户创建成功", code: 200 }
    /// // 英文: { data: Some(user_data), msg: "User created successfully", code: 200 }
    /// ```
    pub fn success_i18n(data: T, message_key: &str) -> Self {
        Self {
            data: Some(data),
            message: translate_key(message_key),
            code: http_status::SUCCESS,
        }
    }

    /// 创建成功响应（无数据 + i18n 消息键）
    /// 
    /// # 示例
    /// ```
    /// let response = ApiResponse::ok_i18n("user-deleted");
    /// // 中文: { data: None, msg: "用户删除成功", code: 200 }
    /// ```
    pub fn ok_i18n(message_key: &str) -> Self {
        Self {
            data: None,
            message: translate_key(message_key),
            code: http_status::SUCCESS,
        }
    }
}

impl ApiResponse<()> {
    /// 创建错误响应（只有消息）
    /// 
    /// 默认: code = 500
    /// 
    /// # 示例
    /// ```
    /// let response = ApiResponse::error("操作失败");
    /// // { data: None, msg: "操作失败", code: 500 }
    /// ```
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            data: None,
            message: message.into(),
            code: http_status::INTERNAL_ERROR,
        }
    }

    /// 创建错误响应（消息 + 状态码）
    /// 
    /// # 示例
    /// ```
    /// let response = ApiResponse::error_code("未找到", 404);
    /// // { data: None, msg: "未找到", code: 404 }
    /// ```
    pub fn error_code(message: impl Into<String>, code: i32) -> Self {
        Self {
            data: None,
            message: message.into(),
            code,
        }
    }

    /// 常用错误：参数错误 (400)
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::error_code(message, http_status::BAD_REQUEST)
    }

    /// 常用错误：未授权 (401)
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::error_code(message, http_status::UNAUTHORIZED)
    }

    /// 常用错误：禁止访问 (403)
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::error_code(message, http_status::FORBIDDEN)
    }

    /// 常用错误：未找到 (404)
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::error_code(message, http_status::NOT_FOUND)
    }

    // ============= i18n 错误方法（自动翻译） =============

    /// 创建错误响应（i18n 消息键）
    /// 
    /// 自动根据当前语言翻译消息，默认 500 错误码
    /// 
    /// # 示例
    /// ```
    /// let response = ApiResponse::error_i18n("db-error");
    /// // 中文: { data: None, msg: "数据库操作失败", code: 500 }
    /// // 英文: { data: None, msg: "Database operation failed", code: 500 }
    /// ```
    pub fn error_i18n(message_key: &str) -> Self {
        Self {
            data: None,
            message: translate_key(message_key),
            code: http_status::INTERNAL_ERROR,
        }
    }

    /// 创建错误响应（i18n 消息键 + 自定义状态码）
    pub fn error_i18n_code(message_key: &str, code: i32) -> Self {
        Self {
            data: None,
            message: translate_key(message_key),
            code,
        }
    }

    /// 常用错误：参数错误 (400) + i18n
    pub fn bad_request_i18n(message_key: &str) -> Self {
        Self::error_i18n_code(message_key, http_status::BAD_REQUEST)
    }

    /// 常用错误：未授权 (401) + i18n
    pub fn unauthorized_i18n(message_key: &str) -> Self {
        Self::error_i18n_code(message_key, http_status::UNAUTHORIZED)
    }

    /// 常用错误：禁止访问 (403) + i18n
    pub fn forbidden_i18n(message_key: &str) -> Self {
        Self::error_i18n_code(message_key, http_status::FORBIDDEN)
    }

    /// 常用错误：未找到 (404) + i18n
    pub fn not_found_i18n(message_key: &str) -> Self {
        Self::error_i18n_code(message_key, http_status::NOT_FOUND)
    }
}

impl<T> Default for ApiResponse<T> {
    fn default() -> Self {
        Self {
            data: None,
            message: constants::SUCCESS.to_string(),
            code: http_status::SUCCESS,
        }
    }
}
