// 数据仓储层实现 - 基于 rbatis 的数据访问层
use async_trait::async_trait;
use football_common::AppResult;

/// 基础仓储 trait
#[async_trait]
pub trait BaseRepository<T> {
    async fn find_by_id(&self, id: i64) -> AppResult<Option<T>>;
    async fn find_all(&self) -> AppResult<Vec<T>>;
    async fn insert(&self, entity: &T) -> AppResult<u64>;
    async fn update(&self, entity: &T) -> AppResult<u64>;
    async fn delete_by_id(&self, id: i64) -> AppResult<u64>;
}


