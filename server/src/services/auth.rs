use std::ops::Add;

use salvo::{
    http::cookie::time::{Duration, OffsetDateTime},
    jwt_auth::{ConstDecoder, HeaderFinder},
    prelude::*,
    Depot,
};
use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Result};

use crate::common::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub userid: i32,
    pub exp: i64,
}

impl JwtClaims {
    pub fn with_exp_days(userid: i32, days: i64) -> Self {
        Self {
            userid: userid,
            exp: OffsetDateTime::now_utc()
                .add(Duration::days(days))
                .unix_timestamp(),
        }
    }
}

pub fn get_auth_handler() -> JwtAuth<JwtClaims, ConstDecoder> {
    JwtAuth::new(ConstDecoder::from_secret(&CONFIG.jwt_secret.as_bytes()))
        .finders(vec![Box::new(HeaderFinder::new())])
        .force_passed(true)
}

type Userid = i32;

pub trait CheckUserAuth {
    fn check_user_auth(&self) -> UserAuthState;
}

impl CheckUserAuth for Depot {
    fn check_user_auth(&self) -> UserAuthState {
        match self.jwt_auth_state() {
            JwtAuthState::Authorized => {
                let claims = &self.jwt_auth_data::<JwtClaims>().unwrap().claims;
                if check_expired(claims.exp) {
                    UserAuthState::Authorized(claims.userid)
                } else {
                    UserAuthState::Expired
                }
            }
            JwtAuthState::Unauthorized => UserAuthState::Unauthorized,
            JwtAuthState::Forbidden => UserAuthState::Forbidden,
        }
    }
}

pub enum UserAuthState {
    Authorized(Userid),
    Expired,
    Unauthorized,
    Forbidden,
}

fn check_expired(exp: i64) -> bool {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    now < exp
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResponse {
    pub token: String,
}

impl TokenResponse {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
