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
    pub fn verify_password(&self, password: &str) -> bool {
        self.password == password
    }
}
