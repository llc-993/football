// 支持国际化的响应辅助模块
use crate::common::response::{ApiResponse, http_status};

/// 创建国际化错误响应
/// 
/// # 示例
/// ```
/// // 在 handler 中使用
/// return HttpResponse::NotFound().json(i18n_error(lang, i18n, "user-not-found"));
/// ```
pub fn i18n_error<I18n>(lang: impl Into<i18n::Language>, i18n: &I18n, key: &str) -> ApiResponse<()>
where
    I18n: I18nProvider,
{
    let message = i18n.get(lang.into(), key);
    ApiResponse::error(message)
}

/// 创建国际化成功响应
pub fn i18n_success<T, I18n>(lang: impl Into<i18n::Language>, i18n: &I18n, data: T, key: &str) -> ApiResponse<T>
where
    I18n: I18nProvider,
{
    let message = i18n.get(lang.into(), key);
    ApiResponse::success_msg(data, message)
}

/// I18n Provider trait（用于泛型约束）
pub trait I18nProvider {
    fn get(&self, lang: i18n::Language, key: &str) -> String;
}

// 占位模块，实际由使用方定义
pub mod i18n {
    #[derive(Debug, Clone, Copy)]
    pub enum Language {
        ZhCN,
        En,
    }
}

