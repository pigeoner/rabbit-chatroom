use anyhow::{anyhow, Result};

pub use crate::entity::user::types::{
    UserError, UserLogin, UserResult, UserSignup, Userid, Userinfo,
};
use crate::utils::SqlModel;

pub struct UserHandler {
    db_handler: SqlModel,
}

impl UserHandler {
    pub async fn new() -> Result<UserHandler> {
        let db_handler = SqlModel::new().await?;
        Ok(Self { db_handler })
    }

    pub async fn signup(&mut self, user_signup: &UserSignup) -> UserResult<()> {
        self.db_handler.insert(user_signup).await?;
        Ok(())
    }

    pub async fn login(&mut self, user_login: &UserLogin) -> UserResult<Userid> {
        let db_user = self.db_handler.select_by_name(&user_login.username).await?;
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
        let db_user = self.db_handler.select_by_id(userid).await?;
        match db_user {
            Some(db_user) => Ok(db_user.into()),
            None => Err(UserError::UserNotFound),
        }
    }

    pub async fn get_userinfo_by_name(&mut self, username: &str) -> UserResult<Userinfo> {
        let db_user = self.db_handler.select_by_name(username).await?;
        match db_user {
            Some(db_user) => Ok(db_user.into()),
            None => Err(UserError::UserNotFound),
        }
    }

    pub async fn update_userinfo(&mut self, userid: i32, new_info: Userinfo) -> UserResult<()> {
        self.db_handler.update_info_by_id(userid, new_info).await?;
        Ok(())
    }
}
