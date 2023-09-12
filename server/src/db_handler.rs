use async_once::AsyncOnce;
use lazy_static::lazy_static;

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{pool::PoolConnection, sqlite::SqlitePool, Sqlite};
use sqlx::{FromRow, Row};
use sqlx::Error as SqlxError;

use crate::config::CONFIG;
use crate::user_type::{User, UserError, UserLogin, GetUserLoginFields};

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
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            avatar_url TEXT
        )",
    )
    .execute(&pool)
    .await
    .unwrap();
}

type SqlxResult<T> = Result<T, SqlxError>;

#[derive(Clone)]
pub struct DBHandler {
    pool: SqlitePool,
}

impl DBHandler {
    pub async fn new() -> Self {
        let pool = POOL.get().await.clone();
        Self { pool }
    }

    pub async fn init_db(&self) -> SqlxResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                avatar_url TEXT
            )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_user(&self, user: &impl GetUserLoginFields) -> SqlxResult<()> {
        sqlx::query("INSERT INTO users (username, password) VALUES (?1, ?2)")
            .bind(user.get_username())
            .bind(user.get_password())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_user(&self, username: &str) -> SqlxResult<Option<User>> {
        let row = sqlx::query("SELECT * FROM users WHERE username = ?1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => {
                let user = User::from_row(&row)?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
}
