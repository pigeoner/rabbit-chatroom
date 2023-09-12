pub mod types;

use types::UserError;
use types::UserLogin;

use crate::db_handler::DBHandler;

use anyhow::{anyhow, Result};

use self::types::UserResult;
use self::types::UserSignup;

pub struct UserHandler {
    db_handler: DBHandler,
}

impl UserHandler {
    pub async fn new() -> Result<UserHandler> {
        let db_handler = DBHandler::new().await?;
        Ok(Self { db_handler })
    }

    pub async fn signup(&mut self, user_signup: &UserSignup) -> UserResult<()> {
        self.db_handler.insert_user(user_signup).await?;
        Ok(())
    }

    pub async fn login(&mut self, user_login: &UserLogin) -> UserResult<()> {
        let db_user = self.db_handler.get_user(&user_login.username).await?;
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
