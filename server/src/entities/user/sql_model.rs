use crate::utils::sql_prelude::*;

use super::types::{User, UserSignup, Userinfo};

impl SqlModel {
    pub async fn insert(&mut self, user: &UserSignup) -> SqlxResult<()> {
        let query = sqlx::query("INSERT INTO users (username, password) VALUES (?1, ?2)")
            .bind(&user.username)
            .bind(&user.password);

        self.conn.execute(query).await?;

        Ok(())
    }

    pub async fn select_by_name(&mut self, username: &str) -> SqlxResult<Option<User>> {
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

    pub async fn select_by_id(&mut self, userid: i32) -> SqlxResult<Option<User>> {
        let query = sqlx::query("SELECT * FROM users WHERE userid = ?1").bind(userid);

        let row = self.conn.fetch_optional(query).await?;

        match row {
            Some(row) => {
                let user = User::from_row(&row)?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    pub async fn update_info_by_id(&mut self, userid: i32, new_info: Userinfo) -> SqlxResult<()> {
        let query = sqlx::query("
UPDATE users
SET username = ?2,
    gender = ?3,
    birthdate = ?4,
    description = ?5,
    avatar = ?6
WHERE
    userid = ?1
        ")
        .bind(userid)
        .bind(&new_info.username)
        .bind(&new_info.gender)
        .bind(&new_info.birthdate)
        .bind(&new_info.description)
        .bind(&new_info.avatar);

    self.conn.execute(query).await?;

    Ok(())
    }
}
