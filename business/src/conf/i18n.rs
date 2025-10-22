// 国际化支持模块 (i18n) - 使用 Fluent 框架
// 
// 解决方案：预加载 FluentResource（线程安全），在使用时创建临时 Bundle
use fluent::{FluentBundle, FluentResource};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::cell::RefCell;
use unic_langid::LanguageIdentifier;

// 重新导出 FluentArgs 以便使用
pub use fluent_bundle::FluentArgs;

// Thread-local 存储当前请求的语言
thread_local! {
    static CURRENT_LANGUAGE: RefCell<Language> = RefCell::new(Language::ZhCN);
}

/// 支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// 中文简体
    ZhCN,
    /// 英语
    En,
}

impl Language {
    /// 从字符串解析语言
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "zh" | "zh-cn" | "zh_cn" | "chinese" => Language::ZhCN,
            "en" | "en-us" | "en_us" | "english" => Language::En,
            _ => Language::ZhCN, // 默认中文
        }
    }

    /// 从 HTTP Accept-Language 头解析
    pub fn from_accept_language(header: &str) -> Self {
        if let Some(lang) = header.split(',').next() {
            let lang = lang.split(';').next().unwrap_or(lang).trim();
            Self::from_str(lang)
        } else {
            Language::ZhCN
        }
    }

    /// 获取语言标识符
    fn lang_id(&self) -> LanguageIdentifier {
        match self {
            Language::ZhCN => "zh-CN".parse().expect("Invalid language identifier"),
            Language::En => "en-US".parse().expect("Invalid language identifier"),
        }
    }

    /// 获取 Fluent 文件名
    fn fluent_file(&self) -> &'static str {
        match self {
            Language::ZhCN => "zh-CN.ftl",
            Language::En => "en-US.ftl",
        }
    }
}

/// 消息键常量
pub mod keys {
    // 通用消息
    pub const SUCCESS: &str = "common-success";
    pub const FAIL: &str = "common-fail";
    pub const INTERNAL_ERROR: &str = "common-internal-error";
    
    // 用户相关
    pub const USER_NOT_FOUND: &str = "user-not-found";
    pub const USER_CREATED: &str = "user-created";
    pub const USER_UPDATED: &str = "user-updated";
    pub const USER_DELETED: &str = "user-deleted";
    pub const USER_ALREADY_EXISTS: &str = "user-already-exists";
    pub const USER_FOUND: &str = "user-found";
    pub const USER_WELCOME: &str = "user-welcome";
    
    // 认证相关
    pub const UNAUTHORIZED: &str = "auth-unauthorized";
    pub const FORBIDDEN: &str = "auth-forbidden";
    pub const INVALID_TOKEN: &str = "auth-invalid-token";
    pub const LOGIN_SUCCESS: &str = "auth-login-success";
    pub const LOGIN_FAILED: &str = "auth-login-failed";
    
    // 验证相关
    pub const VALIDATION_ERROR: &str = "validation-error";
    pub const INVALID_PARAM: &str = "validation-invalid-param";
    pub const REQUIRED_FIELD: &str = "validation-required-field";
    
    // 数据库相关
    pub const DB_ERROR: &str = "db-error";
    pub const DB_CONNECTION_FAILED: &str = "db-connection-failed";
    
    // Redis 相关
    pub const REDIS_ERROR: &str = "redis-error";
    pub const REDIS_CONNECTION_FAILED: &str = "redis-connection-failed";
    
    // 缓存相关
    pub const CACHE_MISS: &str = "cache-miss";
    
    // 操作相关
    pub const OPERATION_SUCCESS: &str = "operation-success";
    pub const OPERATION_FAILED: &str = "operation-failed";
    
    // 用户相关
    pub const USER_REGISTER_SUCCESS: &str = "user-register-success";
    pub const USER_LOGIN_SUCCESS: &str = "user-login-success";
    pub const USER_PASSWORD_CHANGED: &str = "user-password-changed";
    pub const USER_PASSWORD_RESET: &str = "user-password-reset";
    pub const USER_DETAIL_SUCCESS: &str = "user-detail-success";
    
    // 用户错误
    pub const EMAIL_ALREADY_EXISTS: &str = "email-already-exists";
    pub const USERNAME_OR_PASSWORD_ERROR: &str = "username-or-password-error";
    pub const ACCOUNT_FROZEN_OR_DISABLED: &str = "account-frozen-or-disabled";
    pub const OLD_PASSWORD_ERROR: &str = "old-password-error";
    pub const EMAIL_NOT_REGISTERED: &str = "email-not-registered";
    pub const VERIFICATION_CODE_ERROR: &str = "verification-code-error";
    
    // 参数校验错误
    pub const PASSWORD_REQUIRED: &str = "password-required";
    pub const EMAIL_OR_PHONE_REQUIRED: &str = "email-or-phone-required";
    
    // 系统错误
    pub const TOKEN_GENERATION_FAILED: &str = "token-generation-failed";
    pub const UPDATE_LOGIN_TIME_FAILED: &str = "update-login-time-failed";
    
    // 复数支持
    pub const ITEMS_COUNT: &str = "items-count";
}

/// 加载 Fluent Resource（资源是线程安全的）
fn load_resource(lang: Language) -> Arc<FluentResource> {
    // 使用 include_str! 在编译时嵌入文件内容
    let ftl_string = match lang {
        Language::ZhCN => include_str!("../../locales/zh-CN.ftl"),
        Language::En => include_str!("../../locales/en-US.ftl"),
    };
    
    let resource = FluentResource::try_new(ftl_string.to_string()).unwrap_or_else(|e| {
        log::error!("解析 Fluent 资源失败: {:?}", e);
        FluentResource::try_new(get_default_ftl(lang)).expect("Default FTL should be valid")
    });
    
    Arc::new(resource)
}

/// 获取默认 FTL 内容（当文件加载失败时使用）
fn get_default_ftl(lang: Language) -> String {
    match lang {
        Language::ZhCN => {
            r#"
common-success = 操作成功
common-fail = 操作失败
user-not-found = 用户不存在
"#.to_string()
        }
        Language::En => {
            r#"
common-success = Success
common-fail = Failed
user-not-found = User not found
"#.to_string()
        }
    }
}

/// 全局 Fluent Resources（线程安全，只包含资源数据）
static RESOURCES: Lazy<HashMap<Language, Arc<FluentResource>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(Language::ZhCN, load_resource(Language::ZhCN));
    map.insert(Language::En, load_resource(Language::En));
    
    log::info!("Fluent 多语言资源加载成功");
    map
});

/// 创建临时的 FluentBundle（每次使用时创建）
fn create_bundle(lang: Language) -> FluentBundle<Arc<FluentResource>> {
    let resource = RESOURCES.get(&lang)
        .expect("Resource not found")
        .clone();
    
    let lang_id = lang.lang_id();
    let mut bundle = FluentBundle::new(vec![lang_id]);
    
    if let Err(e) = bundle.add_resource(resource) {
        log::error!("添加 Fluent 资源失败: {:?}", e);
    }
    
    bundle
}

/// Fluent i18n 工具（线程安全，可以在应用状态中共享）
#[derive(Clone)]
pub struct I18nBundles;

impl I18nBundles {
    /// 初始化（触发资源加载）
    pub fn new() -> Self {
        // 强制初始化 RESOURCES
        Lazy::force(&RESOURCES);
        Self
    }

    /// 获取消息（无参数）
    pub fn get(&self, lang: Language, key: &str) -> String {
        let bundle = create_bundle(lang);
        
        let message = match bundle.get_message(key) {
            Some(msg) => msg,
            None => {
                log::warn!("消息键未找到: {}", key);
                return key.to_string();
            }
        };
        
        let pattern = match message.value() {
            Some(p) => p,
            None => {
                log::warn!("消息值未找到: {}", key);
                return key.to_string();
            }
        };
        
        let mut errors = vec![];
        let value = bundle.format_pattern(pattern, None, &mut errors);
        
        if !errors.is_empty() {
            log::warn!("格式化消息时出错: {:?}", errors);
        }
        
        value.to_string()
    }

    /// 获取消息（带参数）
    pub fn get_with_args(&self, lang: Language, key: &str, args: Option<&FluentArgs>) -> String {
        let bundle = create_bundle(lang);
        
        let message = match bundle.get_message(key) {
            Some(msg) => msg,
            None => {
                log::warn!("消息键未找到: {}", key);
                return key.to_string();
            }
        };
        
        let pattern = match message.value() {
            Some(p) => p,
            None => {
                log::warn!("消息值未找到: {}", key);
                return key.to_string();
            }
        };
        
        let mut errors = vec![];
        let value = bundle.format_pattern(pattern, args, &mut errors);
        
        if !errors.is_empty() {
            log::warn!("格式化消息时出错: {:?}", errors);
        }
        
        value.to_string()
    }

    /// 便捷方法：获取带单个参数的消息
    pub fn get_with_arg(&self, lang: Language, key: &str, arg_name: &str, arg_value: &str) -> String {
        let mut args = FluentArgs::new();
        args.set(arg_name, arg_value);
        self.get_with_args(lang, key, Some(&args))
    }

    /// 便捷方法：获取带数字参数的消息（用于复数）
    pub fn get_with_count(&self, lang: Language, key: &str, count: i32) -> String {
        let mut args = FluentArgs::new();
        args.set("count", count);
        self.get_with_args(lang, key, Some(&args))
    }
}

impl Default for I18nBundles {
    fn default() -> Self {
        Self::new()
    }
}

// ============= 全局 API（Thread-local 方式） =============

/// 设置当前线程的语言
pub fn set_current_language(lang: Language) {
    CURRENT_LANGUAGE.with(|current| {
        *current.borrow_mut() = lang;
    });
}

/// 获取当前线程的语言
pub fn get_current_language() -> Language {
    CURRENT_LANGUAGE.with(|current| *current.borrow())
}

/// 全局翻译函数：根据当前线程语言翻译消息键
/// 
/// # 示例
/// ```
/// use crate::conf::i18n::{translate, keys};
/// let msg = translate(keys::SUCCESS);
/// ```
pub fn translate(key: &str) -> String {
    let lang = get_current_language();
    let i18n = I18nBundles::new();
    i18n.get(lang, key)
}

/// 全局翻译函数（带参数）
pub fn translate_with_args(key: &str, args: Option<&FluentArgs>) -> String {
    let lang = get_current_language();
    let i18n = I18nBundles::new();
    i18n.get_with_args(lang, key, args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_str() {
        assert_eq!(Language::from_str("zh-CN"), Language::ZhCN);
        assert_eq!(Language::from_str("en"), Language::En);
        assert_eq!(Language::from_str("unknown"), Language::ZhCN);
    }

    #[test]
    fn test_get_message() {
        let i18n = I18nBundles::new();
        let msg_zh = i18n.get(Language::ZhCN, keys::SUCCESS);
        assert!(!msg_zh.is_empty());
        
        let msg_en = i18n.get(Language::En, keys::SUCCESS);
        assert!(!msg_en.is_empty());
    }

    #[test]
    fn test_get_with_args() {
        let i18n = I18nBundles::new();
        let msg = i18n.get_with_arg(Language::ZhCN, keys::USER_WELCOME, "username", "测试用户");
        assert!(msg.contains("测试用户"));
    }

    #[test]
    fn test_plurals() {
        let i18n = I18nBundles::new();
        let msg0 = i18n.get_with_count(Language::En, keys::ITEMS_COUNT, 0);
        let msg1 = i18n.get_with_count(Language::En, keys::ITEMS_COUNT, 1);
        let msg5 = i18n.get_with_count(Language::En, keys::ITEMS_COUNT, 5);
        
        assert!(msg0.contains("No items"));
        assert!(msg1.contains("1 item"));
        assert!(msg5.contains("5 items"));
    }
}
