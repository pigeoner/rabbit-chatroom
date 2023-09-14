use std::ops::Add;

use anyhow::{anyhow, Result};
use salvo::{
    http::cookie::time::{Duration, OffsetDateTime},
    jwt_auth::{ConstDecoder, HeaderFinder},
    prelude::*,
    Depot,
};
use serde::{Deserialize, Serialize};

use crate::common::CONFIG;
use crate::controller::utils::RenderMsg;

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

pub fn get_auth_hoop() -> JwtAuth<JwtClaims, ConstDecoder> {
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
                let claims = match self.jwt_auth_data::<JwtClaims>() {
                    Some(data) => &data.claims,
                    None => return UserAuthState::Unauthorized,
                };
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

impl UserAuthState {
    pub fn is_authorized(&self) -> bool {
        match self {
            UserAuthState::Authorized(_) => true,
            _ => false,
        }
    }
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

#[handler]
pub async fn check_user_authed(
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> Result<()> {
    let auth_state = depot.check_user_auth();

    match auth_state {
        UserAuthState::Authorized(userid) => {
            depot.insert("authed_userid", userid);
        }
        UserAuthState::Expired => {
            res.render_statuscoded_msg(StatusCode::UNAUTHORIZED, "登录已过期")
        }
        UserAuthState::Unauthorized => {
            res.render_statuscoded_msg(StatusCode::UNAUTHORIZED, "用户未登录")
        }
        UserAuthState::Forbidden => res.render_statuscoded_msg(StatusCode::FORBIDDEN, "用户无权限"),
    };

    if !auth_state.is_authorized() {
        ctrl.skip_rest();
    }

    Ok(())
}
