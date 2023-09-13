use anyhow::Result;
use async_once::AsyncOnce;
use lazy_static::lazy_static;

use crate::common::CONFIG;

pub mod prelude {
    pub use sqlx::sqlite::SqliteConnectOptions;
    pub use sqlx::{pool::PoolConnection, sqlite::SqlitePool, Sqlite};
    pub use sqlx::{Error as SqlxError, Executor};
    pub use sqlx::{FromRow, Row};

    pub use super::SqlModel;

    pub type SqlxResult<T> = Result<T, SqlxError>;
}

use prelude::*;

lazy_static! {
    static ref POOL: AsyncOnce<SqlitePool> = AsyncOnce::new(async {
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::new()
                .filename(&CONFIG.database_url)
                .create_if_missing(true),
        )
        .await
        .unwrap();
        init_db(pool.clone()).await;
        pool
    });
}

async fn init_db(pool: SqlitePool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            userid INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            gender TEXT,
            birthdate TEXT,
            description TEXT,
            avatar TEXT
        )",
    )
    .execute(&pool)
    .await
    .unwrap();
}

pub struct SqlModel {
    pub conn: PoolConnection<Sqlite>,
}

impl SqlModel {
    pub async fn new() -> Result<Self> {
        let conn = POOL.get().await.acquire().await?;
        Ok(Self { conn })
    }
}
