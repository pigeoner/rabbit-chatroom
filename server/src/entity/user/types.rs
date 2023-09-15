use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Extractible, Debug, FromRow)]
pub struct User {
    pub userid: i32,
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
#[salvo(extract(default_source(from = "body")))]
pub struct UserSignup {
    pub username: String,
    pub password: String,
    pub uuid: String,
    pub verifycode: String,
}

#[derive(Serialize, Deserialize, Extractible, Debug, Clone)]
pub struct Userinfo {
    pub userid: i32,
    pub username: String,
    pub gender: Option<String>,
    pub birthdate: Option<String>,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

impl From<User> for Userinfo {
    fn from(user: User) -> Userinfo {
        Userinfo {
            userid: user.userid,
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
