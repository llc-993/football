use thiserror::Error;
use actix_web::{error::ResponseError, HttpResponse};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    DatabaseError(String),

    #[error("Redis错误: {0}")]
    RedisError(String),

    #[error("未找到资源: {0}")]
    NotFound(String),

    #[error("参数验证错误: {0}")]
    ValidationError(String),

    #[error("未授权")]
    Unauthorized,

    #[error("禁止访问")]
    Forbidden,

    #[error("内部服务器错误: {0}")]
    InternalServerError(String),

    #[error("业务错误: {0}")]
    BusinessError(String),
}

pub type AppResult<T> = Result<T, AppError>;

// 实现 Actix Web 的 ResponseError trait
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(msg) => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "NotFound",
                    "message": msg
                }))
            }
            AppError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "ValidationError",
                    "message": msg
                }))
            }
            AppError::Unauthorized => {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Unauthorized",
                    "message": "未授权访问"
                }))
            }
            AppError::Forbidden => {
                HttpResponse::Forbidden().json(serde_json::json!({
                    "error": "Forbidden",
                    "message": "禁止访问"
                }))
            }
            AppError::DatabaseError(msg) | 
            AppError::RedisError(msg) | 
            AppError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "InternalServerError",
                    "message": msg
                }))
            }
            AppError::BusinessError(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "BusinessError",
                    "message": msg
                }))
            }
        }
    }
}

// 从 rbatis 错误转换
impl From<rbatis::Error> for AppError {
    fn from(err: rbatis::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

// 从 redis 错误转换
impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::RedisError(err.to_string())
    }
}

