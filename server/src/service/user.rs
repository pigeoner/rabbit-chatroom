use anyhow::Result;
use crate::entity::types::SqlxError;

pub use crate::entity::user::types::{
    UserLogin, UserSignup, Userinfo,
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

    pub async fn update_password(
        &mut self,
        userid: i32,
        old_password: &str,
        new_password: &str,
    ) -> UserResult<()> {
        let db_user = self.db_handler.select_by_id(userid).await?;
        match db_user {
            Some(db_user) => {
                if db_user.password == old_password {
                    self.db_handler
                        .update_password_by_id(userid, new_password)
                        .await?;
                    Ok(())
                } else {
                    Err(UserError::PasswordNotMatch)
                }
            }
            None => Err(UserError::UserNotFound),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserError {
    #[error("用户不存在")]
    UserNotFound,
    #[error("密码错误")]
    PasswordNotMatch,
    #[error("用户名已存在")]
    UsernameAlreadyExists,
    #[error("其他错误：{0}")]
    Other(anyhow::Error),
}

pub type UserResult<T> = Result<T, UserError>;
type Userid = i32;

impl From<SqlxError> for UserError {
    fn from(e: SqlxError) -> Self {
        match e {
            SqlxError::RowNotFound => UserError::UserNotFound,
            SqlxError::Database(db_err) => {
                if db_err.is_unique_violation() {
                    UserError::UsernameAlreadyExists
                } else {
                    UserError::Other(anyhow::Error::from(db_err))
                }
            }
            _ => UserError::Other(anyhow::Error::from(e)),
        }
    }
}