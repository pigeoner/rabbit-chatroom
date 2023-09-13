use anyhow::{anyhow, Result};
use jsonwebtoken::EncodingKey;
use salvo::{
    http::cookie::time::{Duration, OffsetDateTime},
    prelude::*,
};

use crate::{
    entities::{
        user::{
            types::{UserError, UserLogin, UserSignup},
            UserHandler,
        },
        verifycode::gen_verifycode_base64,
    },
    utils::CONFIG,
};

use super::JwtClaims;

#[handler]
pub async fn get_verifycode(res: &mut Response) -> Result<()> {
    let vr = gen_verifycode_base64().await?;
    res.render(Json(vr));
    Ok(())
}

#[handler]
pub async fn signup(req: &mut Request, res: &mut Response) -> Result<()> {
    log::debug!("new signup: {:?}", req);

    let mut uh = UserHandler::new().await?;
    let user_signup: UserSignup = req.parse_json().await.unwrap();

    if !user_signup.verify().await? {
        res.render(StatusError::bad_request().brief("验证码错误"));
    } else if let Err(e) = uh.signup(&user_signup).await {
        match e {
            UserError::UsernameAlreadyExists => {
                res.render(StatusError::conflict().brief("用户名已存在"));
            }
            UserError::Other(e) => {
                res.render(StatusError::internal_server_error().brief(&e.to_string()));
            }
            _ => {
                res.render(StatusError::internal_server_error().brief("意料外的错误"));
            }
        }
    } else {
        res.render("注册成功");
    }

    Ok(())
}

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) -> Result<()> {
    log::debug!("new login: {:?}", req);

    let mut uh = UserHandler::new().await?;
    let user_login: UserLogin = req.parse_json().await.unwrap();

    if let Err(e) = uh.login(&user_login).await {
        match e {
            UserError::UserNotFound => {
                res.render(StatusError::unauthorized().brief("用户不存在"));
            }
            UserError::PasswordNotMatch => {
                res.render(StatusError::unauthorized().brief("密码错误"));
            }
            UserError::Other(e) => {
                res.render(StatusError::internal_server_error().brief(&e.to_string()));
            }
            _ => {
                res.render(StatusError::internal_server_error().brief("意料外的错误"));
            }
        }
    } else {
        let exp = OffsetDateTime::now_utc() + Duration::days(7);
        let claims = JwtClaims {
            username: user_login.username,
            exp: exp.unix_timestamp(),
        };
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &EncodingKey::from_secret(&CONFIG.jwt_secret.as_bytes()),
        )?;
        res.render(token);
    }
    Ok(())
}

#[handler]
pub async fn get_user_info(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    todo!()
}

#[handler]
pub async fn update_user_info(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    todo!()
}

#[handler]
pub async fn upload_avatar(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    todo!()
}
