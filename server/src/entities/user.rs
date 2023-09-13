mod avatar;
mod sql_model;

pub mod types;

use types::{UserError, UserLogin, UserResult, UserSignup};

use crate::utils::SqlModel;

use anyhow::{anyhow, Result};

use self::types::{Userinfo, Userid};

pub struct UserHandler {
    db_handler: SqlModel,
}

impl UserHandler {
    pub async fn new() -> Result<UserHandler> {
        let db_handler = SqlModel::new().await?;
        Ok(Self { db_handler })
    }

    pub async fn signup(&mut self, user_signup: &UserSignup) -> UserResult<()> {
        self.db_handler.insert_user(user_signup).await?;
        Ok(())
    }

    pub async fn login(&mut self, user_login: &UserLogin) -> UserResult<Userid> {
        let db_user = self.db_handler.get_user_by_name(&user_login.username).await?;
        match db_user {
            Some(db_user) => {
                if user_login.validate(&db_user.password) {
                    Ok(db_user.userid)
                } else {
                    Err(UserError::PasswordNotMatch)
                }
            }
            None => Err(UserError::UserNotFound),
        }
    }

    pub async fn get_userinfo(&mut self, userid: i32) -> UserResult<Userinfo> {
        let db_user = self.db_handler.get_user_by_id(userid).await?;
        match db_user {
            Some(db_user) => Ok(db_user.into()),
            None => Err(UserError::UserNotFound),
        }
    }

    pub async fn get_userinfo_by_name(&mut self, username: &str) -> UserResult<Userinfo> {
        let db_user = self.db_handler.get_user_by_name(username).await?;
        match db_user {
            Some(db_user) => Ok(db_user.into()),
            None => Err(UserError::UserNotFound),
        }
    }
}
