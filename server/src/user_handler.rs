use crate::user_type::UserLogin;
use crate::{db_handler::DBHandler, user_type::UserError};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct UserHandler {
    db_handler: DBHandler,
}

impl UserHandler {
    pub async fn new() -> Self {
        let db_handler = DBHandler::new().await;
        Self { db_handler }
    }

    pub async fn signup(&self, user_login: &UserLogin) -> Result<()> {
        self.db_handler
            .insert_user(user_login)
            .await
            .or_else(|e| Err(anyhow!("注册失败：{}", e)))?;
        Ok(())
    }

    pub async fn login(&self, user_login: &UserLogin) -> Result<(), UserError> {
        let db_user = self
            .db_handler
            .get_user(&user_login.username)
            .await
            .or_else(|e| Err(UserError::Other(anyhow!("登录失败：{}", e))))?;
        match db_user {
            Some(db_user) => {
                if user_login.validate(&db_user.password) {
                    Ok(())
                } else {
                    Err(UserError::PasswordNotMatch)
                }
            }
            None => Err(UserError::UserNotFound),
        }
    }
}
