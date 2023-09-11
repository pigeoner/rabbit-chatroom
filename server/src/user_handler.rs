use crate::types::User;
use crate::{db_handler::DBHandler, types::UserError};

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

    pub async fn signup(&self, user: &User) -> Result<()> {
        self.db_handler
            .insert_user(user)
            .await
            .or_else(|e| Err(anyhow!("注册失败：{}", e)))?;
        Ok(())
    }

    pub async fn login(&self, user: &User) -> Result<(), UserError> {
        let db_user = self
            .db_handler
            .get_user(&user.username)
            .await
            .or_else(|e| Err(UserError::Other(anyhow!("登录失败：{}", e))))?;
        match db_user {
            Some(db_user) => {
                if user.validate(&db_user.password) {
                    Ok(())
                } else {
                    Err(UserError::PasswordNotMatch)
                }
            }
            None => Err(UserError::UserNotFound),
        }
    }
}
