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
    common::CONFIG, services::utils::RenderMsg,
};

use super::JwtClaims;
use super::utils::Msg;

#[handler]
pub async fn get_verifycode(res: &mut Response) -> Result<()> {
    let vr = gen_verifycode_base64().await?;
    res.render(Json(vr));
    Ok(())
}

#[handler]
pub async fn signup(req: &mut Request, res: &mut Response) -> Result<()> {
    let mut uh = UserHandler::new().await?;
    let user_signup: UserSignup = req.parse_json().await.unwrap();

    log::debug!("new signup: {:?}", user_signup);

    if !user_signup.verify().await? {
        res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "验证码错误");
    } else if let Err(e) = uh.signup(&user_signup).await {
        match e {
            UserError::UsernameAlreadyExists => {
                res.render_statuscoded_msg(StatusCode::CONFLICT, "用户名已存在");
            }
            UserError::Other(e) => {
                res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
            }
            _ => {
                res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, "意料外的错误");
            }
        }
    } else {
        res.render_msg("注册成功")
    };

    Ok(())
}

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) -> Result<()> {
    let mut uh = UserHandler::new().await?;
    let user_login: UserLogin = req.parse_json().await.unwrap();

    log::debug!("new login: {:?}", user_login);

    if let Err(e) = uh.login(&user_login).await {
        match e {
            UserError::UserNotFound => {
                res.render_statuscoded_msg(StatusCode::UNAUTHORIZED, "用户不存在");
            }
            UserError::PasswordNotMatch => {
                res.render_statuscoded_msg(StatusCode::UNAUTHORIZED, "密码错误");
            }
            UserError::Other(e) => {
                res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
            }
            _ => {
                res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, "意料外的错误");
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
        res.render_msg(&token)
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
