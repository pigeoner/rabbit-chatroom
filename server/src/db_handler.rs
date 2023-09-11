use lazy_static::lazy_static;
use anyhow::{Result, Ok};

use sqlx::{Row, FromRow};
use sqlx::{sqlite::SqlitePool, Sqlite, pool::PoolConnection};

use crate::config::CONFIG;
use crate::types::User;

pub struct DBHandler {
    pool: SqlitePool,
}

impl DBHandler {
    pub async fn new() -> Self {
        let pool = SqlitePool::connect(&CONFIG.database_url).await.unwrap();
        Self {
            pool,
        }
    }

    // pub async fn get_conn(&self) -> Result<PoolConnection<Sqlite>> {
    //     Ok(self.pool.acquire().await?)
    // }

    pub async fn init_db(&self) -> Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                avatar_url TEXT
            )",
        ).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn insert_user(&self, user: &User) -> Result<()> {
        sqlx::query(
            "INSERT INTO users (username, password, avatar_url) VALUES (?1, ?2, ?3)",
        ).bind(&user.username).bind(&user.password).bind(&user.avatar_url).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get_user(&self, username: &str) -> Result<Option<User>> {
        let row = sqlx::query(
            "SELECT * FROM users WHERE username = ?1",
        ).bind(username).fetch_optional(&self.pool).await?;
        
        match row {
            Some(row) => {
                let user = User::from_row(&row)?;
                Ok(Some(user))
            },
            None => Ok(None),
        }
    }
}