use crate::db_handler::DBHandler;
use crate::types::User;

use anyhow::{anyhow, Ok, Result};

pub struct UserHandler {
    db_handler: DBHandler,
}

impl UserHandler {
    pub fn new(db_handler: DBHandler) -> Self {
        Self { db_handler }
    }

    pub async fn signup(&self, user: &User) -> Result<()> {
        self.db_handler
            .insert_user(user)
            .await
            .or_else(|e| Err(anyhow!("注册失败：{}", e)))?;
        Ok(())
    }

    pub async fn login(&self, user: &User) -> Result<()> {
        let db_user = self
            .db_handler
            .get_user(&user.username)
            .await
            .or_else(|e| Err(anyhow!("登录失败：{}", e)))?;
        match db_user {
            Some(db_user) => {
                if user.verify_password(&db_user.password) {
                    Ok(())
                } else {
                    Err(anyhow!("登录失败：密码错误"))
                }
            }
            None => Err(anyhow!("登录失败：用户不存在")),
        }
    }
}
