use anyhow::Result;
use bb8::{Pool, PooledConnection};
use bb8_redis::{bb8::ManageConnection, RedisConnectionManager};
use redis::Commands;

use async_once::AsyncOnce;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref POOL: AsyncOnce<Pool<RedisConnectionManager>> = AsyncOnce::new(async {
        let manager = RedisConnectionManager::new("redis://127.0.0.1:6379").unwrap();
        Pool::builder().build(manager).await.unwrap()
    });
}

pub async fn get_redis_conn() -> Result<PooledConnection<'static, RedisConnectionManager>> {
    let pool = POOL.get().await;
    let conn = pool.get().await?;
    Ok(conn)
}
