use crate::utils::sql_prelude::*;

use super::types::{User, UserSignup};

impl SqlModel {
    pub async fn insert_user(&mut self, user: &UserSignup) -> SqlxResult<()> {
        let query = sqlx::query("INSERT INTO users (username, password) VALUES (?1, ?2)")
            .bind(&user.username)
            .bind(&user.password);

        self.conn.execute(query).await?;

        Ok(())
    }

    pub async fn get_user_by_name(&mut self, username: &str) -> SqlxResult<Option<User>> {
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

    pub async fn get_user_by_id(&mut self, userid: i32) -> SqlxResult<Option<User>> {
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

    // pub async fn update_user(&mut self, userid: i32, new_user: User) -> SqlxResult<()> {
    //     let query = sqlx::query("")
    // }
}
