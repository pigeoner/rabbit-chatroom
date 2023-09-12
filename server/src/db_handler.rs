use async_once::AsyncOnce;
use lazy_static::lazy_static;

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{pool::PoolConnection, sqlite::SqlitePool, Sqlite};
use sqlx::{Error as SqlxError, Executor};
use sqlx::{FromRow, Row};

use anyhow::Result;

use crate::config::CONFIG;
use crate::user::types::{GetUserLoginFields, User, UserError, UserLogin, UserSignup};

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

pub struct DBHandler {
    conn: PoolConnection<Sqlite>,
}

impl DBHandler {
    pub async fn new() -> Result<Self> {
        let conn = POOL.get().await.acquire().await?;
        Ok(Self { conn })
    }

    pub async fn insert_user(&mut self, user: &UserSignup) -> SqlxResult<()> {
        let query = sqlx::query("INSERT INTO users (username, password) VALUES (?1, ?2)")
            .bind(&user.username)
            .bind(&user.password);

        self.conn.execute(query).await?;

        Ok(())
    }

    pub async fn get_user(&mut self, username: &str) -> SqlxResult<Option<User>> {
        let query = sqlx::query("SELECT * FROM users WHERE username = ?1").bind(username);

        let row = self.conn.fetch_optional(query).await?;

        match row {
            Some(row) => {
                let user = User::from_row(&row)?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
}
