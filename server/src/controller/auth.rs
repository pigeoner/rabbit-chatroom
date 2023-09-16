use std::ops::Add;

use anyhow::Result;
use jsonwebtoken::EncodingKey;
use redis::AsyncCommands;
use salvo::{
    http::cookie::time::{Duration, OffsetDateTime},
    jwt_auth::{ConstDecoder, HeaderFinder},
    prelude::*,
    Depot,
};
use serde::{Deserialize, Serialize};

use crate::{common::CONFIG, utils::get_redis_conn};

use super::utils::RenderMsg;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub userid: i32,
    pub exp: i64,
    pub uuid: String,
}

impl JwtClaims {
    pub fn new(userid: i32) -> Self {
        Self {
            userid,
            exp: OffsetDateTime::now_utc().unix_timestamp()
                + Duration::days(CONFIG.exp_days).whole_seconds(),
            uuid: uuid::Uuid::new_v4().to_string(),
        }
    }

    // JwtAuth会自动判断是否exp
    // fn check_expired(&self) -> bool {
    //     let exp = self.exp;
    //     let now = OffsetDateTime::now_utc().unix_timestamp();

    //     now < exp
    // }

    async fn check_valid(&self) -> Result<bool> {
        let mut conn = get_redis_conn().await?;
        let uuid: Option<String> = conn.get(self.userid).await?;

        match uuid {
            Some(uuid) => Ok(uuid == self.uuid),
            None => Ok(false),
        }
    }
}

pub fn get_auth_hoop() -> JwtAuth<JwtClaims, ConstDecoder> {
    JwtAuth::new(ConstDecoder::from_secret(&CONFIG.jwt_secret.as_bytes()))
        .finders(vec![Box::new(HeaderFinder::new())])
        .force_passed(true)
}

type Userid = i32;

async fn check_user_auth(depot: &mut Depot) -> Result<UserAuthState> {
    match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {
            let claims = match depot.jwt_auth_data::<JwtClaims>() {
                Some(data) => &data.claims,
                None => return Ok(UserAuthState::Unauthorized),
            };
            if !claims.check_valid().await? {
                Ok(UserAuthState::Expired)
            } else {
                Ok(UserAuthState::Authorized(claims.userid))
            }
        }
        JwtAuthState::Unauthorized => Ok(UserAuthState::Unauthorized),
        JwtAuthState::Forbidden => Ok(UserAuthState::Forbidden),
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

#[handler]
pub async fn check_user_authed(
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> Result<()> {
    let auth_state = check_user_auth(depot).await?;

    match auth_state {
        UserAuthState::Authorized(userid) => {
            depot.insert("authed_userid", userid);
        }
        UserAuthState::Expired => {
            res.render_statuscoded_msg(StatusCode::UNAUTHORIZED, "登录已失效")
        }
        UserAuthState::Unauthorized => {
            res.render_statuscoded_msg(StatusCode::UNAUTHORIZED, "用户未登录")
        }
        UserAuthState::Forbidden => res.render_statuscoded_msg(StatusCode::FORBIDDEN, "登录已失效"),
    };

    if !auth_state.is_authorized() {
        ctrl.skip_rest();
    }

    Ok(())
}

pub trait HoopAuth {
    fn hoop_auth(self) -> Self;
}

impl HoopAuth for Router {
    fn hoop_auth(self) -> Self {
        let auth_hoop: JwtAuth<JwtClaims, _> = get_auth_hoop();
        self.hoop(auth_hoop).hoop(check_user_authed)
    }
}

pub async fn sign_token(userid: i32) -> Result<String> {
    let claims = JwtClaims::new(userid);

    let mut conn = get_redis_conn().await?;
    conn.set_ex(
        claims.userid,
        &claims.uuid,
        CONFIG.exp_days as usize * 24 * 60 * 60,
    )
    .await?;

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(&CONFIG.jwt_secret.as_bytes()),
    )?;

    Ok(token)
}
