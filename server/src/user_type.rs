use derive_builder::Builder;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Extractible, Debug, FromRow)]
pub struct User {
    pub username: String,
    pub password: String,
    pub gender: Option<String>,
    pub birthdate: Option<String>,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Serialize, Deserialize, Extractible, Debug, Clone)]
#[salvo(extract(default_source(from = "body")))]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

impl UserLogin {
    pub fn validate(&self, password: &str) -> bool {
        self.password == password
    }
}

#[derive(Serialize, Deserialize, Extractible, Debug, Clone)]
pub struct UserInfo {
    pub username: String,
    pub gender: Option<String>,
    pub birthdate: Option<String>,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

impl From<UserLogin> for User {
    fn from(user_login: UserLogin) -> User {
        User {
            username: user_login.username,
            password: user_login.password,
            gender: None,
            birthdate: None,
            description: None,
            avatar: None,
        }
    }
}

impl From<User> for UserInfo {
    fn from(user: User) -> UserInfo {
        UserInfo {
            username: user.username,
            gender: user.gender,
            birthdate: user.birthdate,
            description: user.description,
            avatar: user.avatar,
        }
    }
}

pub trait GetUserLoginFields {
    fn get_username(&self) -> &str;
    fn get_password(&self) -> &str;
}

impl GetUserLoginFields for UserLogin {
    fn get_username(&self) -> &str {
        &self.username
    }

    fn get_password(&self) -> &str {
        &self.password
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
