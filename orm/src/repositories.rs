// 数据仓储层实现 - 基于 rbatis 的数据访问层
use async_trait::async_trait;
use football_common::AppResult;
use crate::db::get_db;

/// 基础仓储 trait
#[async_trait]
pub trait BaseRepository<T> {
    async fn find_by_id(&self, id: i64) -> AppResult<Option<T>>;
    async fn find_all(&self) -> AppResult<Vec<T>>;
    async fn insert(&self, entity: &T) -> AppResult<u64>;
    async fn update(&self, entity: &T) -> AppResult<u64>;
    async fn delete_by_id(&self, id: i64) -> AppResult<u64>;
}

// 示例：用户仓储
use crate::entities::User;

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        Self
    }

    /// 根据用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let rb = get_db();
        let sql = "SELECT * FROM users WHERE username = ? LIMIT 1";
        let result: Result<Option<User>, rbatis::Error> = rb.query_decode(sql, vec![username.into()]).await;
        Ok(result?)
    }

    /// 根据邮箱查找用户
    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let rb = get_db();
        let sql = "SELECT * FROM users WHERE email = ? LIMIT 1";
        let result: Result<Option<User>, rbatis::Error> = rb.query_decode(sql, vec![email.into()]).await;
        Ok(result?)
    }
}

#[async_trait]
impl BaseRepository<User> for UserRepository {
    async fn find_by_id(&self, id: i64) -> AppResult<Option<User>> {
        let rb = get_db();
        let sql = "SELECT * FROM users WHERE id = ? LIMIT 1";
        let result: Result<Option<User>, rbatis::Error> = rb.query_decode(sql, vec![id.into()]).await;
        Ok(result?)
    }

    async fn find_all(&self) -> AppResult<Vec<User>> {
        let rb = get_db();
        let sql = "SELECT * FROM users";
        let result: Result<Vec<User>, rbatis::Error> = rb.query_decode(sql, vec![]).await;
        Ok(result?)
    }

    async fn insert(&self, entity: &User) -> AppResult<u64> {
        let rb = get_db();
        let sql = "INSERT INTO users (username, email, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)";
        let result = rb.exec(sql, vec![
            entity.username.clone().into(),
            entity.email.clone().into(),
            entity.password_hash.clone().into(),
            entity.created_at.to_rfc3339().into(),
            entity.updated_at.to_rfc3339().into(),
        ]).await?;
        Ok(result.rows_affected)
    }

    async fn update(&self, entity: &User) -> AppResult<u64> {
        let rb = get_db();
        let sql = "UPDATE users SET username = ?, email = ?, password_hash = ?, updated_at = ? WHERE id = ?";
        let result = rb.exec(sql, vec![
            entity.username.clone().into(),
            entity.email.clone().into(),
            entity.password_hash.clone().into(),
            entity.updated_at.to_rfc3339().into(),
            entity.id.into(),
        ]).await?;
        Ok(result.rows_affected)
    }

    async fn delete_by_id(&self, id: i64) -> AppResult<u64> {
        let rb = get_db();
        let sql = "DELETE FROM users WHERE id = ?";
        let result = rb.exec(sql, vec![id.into()]).await?;
        Ok(result.rows_affected)
    }
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}

