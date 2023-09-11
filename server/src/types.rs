use derive_builder::Builder;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Extractible, Debug, Builder, FromRow)]
#[salvo(extract(default_source(from = "body")))]
pub struct User {
    pub username: String,
    pub password: String,
    pub avatar_url: Option<String>,
}

impl User {
    pub fn validate(&self, password: &str) -> bool {
        self.password == password
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserError {
    #[error("用户不存在")]
    UserNotFound,
    #[error("密码错误")]
    PasswordNotMatch,
    #[error("其他错误：{0}")]
    Other(anyhow::Error),
}
