// 配置模块
pub mod i18n;
pub mod middleware;

// 重新导出常用类型
pub use i18n::{I18nBundles, Language, keys};
pub use middleware::LanguageMiddleware;

