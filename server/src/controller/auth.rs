/// use access jwt token and refresh token
/// short token for
use anyhow::Result;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use redis::AsyncCommands;
use salvo::{
    http::cookie::{
        time::{Duration, OffsetDateTime},
        Cookie,
    },
    jwt_auth::{CookieFinder, JwtTokenFinder},
    prelude::*,
    Depot,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{common::CONFIG, render_error_skip, utils::get_redis_conn, render_error};

use super::utils::{ErrorRender, RenderError, RenderMsg};

#[async_trait]
pub trait JwtClaims: Serialize + Send + Sync + 'static + DeserializeOwned {
    fn new(userid: i32) -> Self;
    fn encode(&self) -> Result<String>;
    async fn find_token(req: &mut Request) -> Option<String>;
    fn decode(token: &str) -> JwtAuthResult<Self> {
        // let mut validation = Validation::default();
        // validation.required_spec_claims = Default::default();
        let token_message = decode::<Self>(
            token,
            &DecodingKey::from_secret(&CONFIG.jwt_secret.as_bytes()),
            &Validation::default(),
        );

        match token_message {
            Ok(token_message) => Ok(token_message.claims),
            Err(err) => Err(err.into()),
        }
    }
    async fn find_and_decode(req: &mut Request) -> JwtAuthResult<Self> {
        let token = Self::find_token(req)
            .await
            .ok_or(JwtAuthError::Unauthorized)?;
        Self::decode(&token)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaimsAccess {
    pub userid: i32,
    pub exp: i64,
}

#[async_trait]
impl JwtClaims for JwtClaimsAccess {
    fn new(userid: i32) -> Self {
        Self {
            userid,
            exp: OffsetDateTime::now_utc().unix_timestamp()
                + Duration::minutes(CONFIG.jwt_access_exp_minutes).whole_seconds(),
        }
    }

    fn encode(&self) -> Result<String> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            self,
            &EncodingKey::from_secret(&CONFIG.jwt_secret.as_bytes()),
        )?;
        Ok(token)
    }

    async fn find_token(req: &mut Request) -> Option<String> {
        let finder = CookieFinder::new("jwt_access");
        finder.find_token(req).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaimsRefresh {
    pub userid: i32,
    pub exp: i64,
    pub uuid: String,
}

#[async_trait]
impl JwtClaims for JwtClaimsRefresh {
    fn new(userid: i32) -> Self {
        Self {
            userid,
            exp: OffsetDateTime::now_utc().unix_timestamp()
                + Duration::days(CONFIG.jwt_refresh_exp_days).whole_seconds(),
            uuid: gen_uuid(),
        }
    }

    fn encode(&self) -> Result<String> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            self,
            &EncodingKey::from_secret(&CONFIG.jwt_secret.as_bytes()),
        )?;
        Ok(token)
    }

    async fn find_token(req: &mut Request) -> Option<String> {
        let finder = CookieFinder::new("jwt_refresh");
        finder.find_token(req).await
    }
}

impl JwtClaimsRefresh {
    async fn check_valid(&self) -> JwtAuthResult<()> {
        let mut conn = get_redis_conn().await?;
        let uuid: Option<String> = conn.get(self.userid).await.map_err(anyhow::Error::from)?;

        match uuid {
            Some(uuid) => {
                if uuid == self.uuid {
                    Ok(())
                } else {
                    Err(JwtAuthError::Expired)
                }
            }
            None => Err(JwtAuthError::Expired),
        }
    }

    pub async fn sign_uuid(&self) -> Result<()> {
        let mut conn = get_redis_conn().await?;
        conn.set_ex(
            self.userid,
            &self.uuid,
            CONFIG.jwt_refresh_exp_days as usize * 24 * 60 * 60,
        )
        .await?;
        Ok(())
    }
}

#[handler]
async fn jwt_auth(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> Result<()> {
    dbg!(&req);
    match JwtClaimsAccess::find_and_decode(req).await {
        Ok(claims) => {
            depot.insert(AUTHED_USERID_KEY, claims.userid);
        }
        Err(_) => {
            let claims =
                render_error_skip!(JwtClaimsRefresh::find_and_decode(req).await, res, ctrl);

            render_error_skip!(claims.check_valid().await, res, ctrl);

            depot.insert(AUTHED_USERID_KEY, claims.userid);

            render_error!(set_cookie_token(res, claims.userid).await, res);
        }
    };
    Ok(())
}

// async fn find_access_token(req: &mut Request) -> Option<String> {
//     let finder = CookieFinder::new("jwt_access");
//     finder.find_token(req).await
// }

// async fn find_refresh_token(req: &mut Request) -> Option<String> {
//     let finder = CookieFinder::new("jwt_refresh");
//     finder.find_token(req).await
// }

pub const AUTHED_USERID_KEY: &str = "authed_userid";

pub trait HoopAuth {
    fn hoop_auth(self) -> Self;
}
impl HoopAuth for Router {
    fn hoop_auth(self) -> Self {
        self.hoop(jwt_auth)
    }
}

pub async fn set_cookie_token(res: &mut Response, userid: i32) -> Result<()> {
    let claims_access = JwtClaimsAccess::new(userid);
    let claims_refresh = JwtClaimsRefresh::new(userid);

    let token_access = claims_access.encode()?;
    dbg!(&token_access);
    res.add_cookie(
        Cookie::build("jwt_access", token_access)
            // .http_only(true)
            // .secure(true)
            .finish(),
    );

    let token_refresh = claims_refresh.encode()?;
    res.add_cookie(
        Cookie::build("jwt_refresh", token_refresh)
            // .http_only(true)
            // .secure(true)
            .finish(),
    );

    claims_refresh.sign_uuid().await?;

    Ok(())
}

fn gen_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[derive(thiserror::Error, Debug)]
pub enum JwtAuthError {
    #[error("用户未登录")]
    Unauthorized,
    #[error("登录已过期")]
    Expired,
    #[error("无效的登录信息")]
    Forbidden,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
pub type JwtAuthResult<T> = Result<T, JwtAuthError>;

impl From<jsonwebtoken::errors::Error> for JwtAuthError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => JwtAuthError::Forbidden,
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtAuthError::Expired,
            _ => JwtAuthError::Forbidden,
        }
    }
}

impl ErrorRender for JwtAuthError {
    fn error_render(&self, res: &mut Response) {
        match self {
            JwtAuthError::Unauthorized => {
                res.render_statuscoded_msg(StatusCode::UNAUTHORIZED, "用户未登录")
            }
            JwtAuthError::Expired => {
                res.render_statuscoded_msg(StatusCode::UNAUTHORIZED, "登录已过期")
            }
            JwtAuthError::Forbidden => {
                res.render_statuscoded_msg(StatusCode::FORBIDDEN, "无效的登录信息")
            }
            JwtAuthError::Other(err) => err.error_render(res),
        }
    }
}
