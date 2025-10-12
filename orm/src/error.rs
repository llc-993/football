// ORM 层错误处理和转换
use football_common::AppError;

/// 从 rbatis 错误转换为应用错误
pub fn from_rbatis_error(err: rbatis::Error) -> AppError {
    AppError::DatabaseError(format!("数据库错误: {}", err))
}

/// 从 redis 错误转换为应用错误
pub fn from_redis_error(err: redis::RedisError) -> AppError {
    AppError::RedisError(format!("Redis错误: {}", err))
}

