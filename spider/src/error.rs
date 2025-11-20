// 爬虫错误定义
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpiderError {
    #[error("HTTP请求失败: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("解析错误: {0}")]
    ParseError(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON序列化错误: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type SpiderResult<T> = Result<T, SpiderError>;

