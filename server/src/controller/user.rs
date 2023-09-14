use anyhow::{anyhow, Result};
use jsonwebtoken::EncodingKey;
use salvo::prelude::*;

use crate::{
    common::CONFIG,
    service::{
        user::{
            UserError, UserLogin, UserSignup, Userinfo,
            UserHandler,
        },
        verifycode::{gen_verifycode_base64, VerifyResult},
    },
    controller::auth::TokenResponse,
};

use super::{
    utils::RenderMsg,
    JwtClaims,
};

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

    match user_signup.verify().await {
        Err(e) => {
            res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
        }
        Ok(VerifyResult::Fail) => {
            res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "验证码错误");
        }
        Ok(VerifyResult::Expired) => {
            res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "验证码已过期");
        }
        Ok(VerifyResult::Success) => match uh.signup(&user_signup).await {
            Err(e) => match e {
                UserError::UsernameAlreadyExists => {
                    res.render_statuscoded_msg(StatusCode::CONFLICT, "用户名已存在");
                }
                UserError::Other(e) => {
                    res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
                }
                _ => {
                    res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, "意料外的错误");
                }
            },
            Ok(_) => {
                res.render_msg("ok");
            }
        },
    };

    Ok(())
}

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) -> Result<()> {
    let mut uh = UserHandler::new().await?;
    let user_login: UserLogin = req.parse_json().await.unwrap();

    log::debug!("new login: {:?}", user_login);

    match uh.login(&user_login).await {
        Err(e) => 
            match e {
                UserError::UserNotFound => {
                    res.render_statuscoded_msg(StatusCode::NOT_FOUND, "用户不存在");
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
        Ok(userid) => {
            let claims = JwtClaims::with_exp_days(userid, CONFIG.exp_days);
            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &EncodingKey::from_secret(&CONFIG.jwt_secret.as_bytes()),
            )?;
            res.render(Json(TokenResponse::new(token)))
        }
    }

    Ok(())
}

#[handler]
pub async fn get_self_userinfo(depot: &mut Depot, res: &mut Response) -> Result<()> {
    let userid = depot.get::<i32>("authed_userid").unwrap().to_owned();

    let mut uh = UserHandler::new().await?;
    let user_info = uh.get_userinfo(userid).await?;
    res.render(Json(user_info));

    Ok(())
}

#[handler]
pub async fn get_userinfo(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let userid: i32 = match req.param("userid") {
        Some(userid) => userid,
        None => {
            res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "无效的userid");
            return Ok(());
        }
    };

    let mut uh = UserHandler::new().await?;
    let user_info = uh.get_userinfo(userid).await?;
    res.render(Json(user_info));

    Ok(())
}

#[handler]
pub async fn update_userinfo(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    let userid = depot.get::<i32>("authed_userid").unwrap().to_owned();

    let mut uh = UserHandler::new().await?;
    let new_info: Userinfo = req.parse_json().await.unwrap();

    if userid != new_info.userid {
        res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "无效的userid");
        return Ok(());
    }

    uh.update_userinfo(userid, new_info).await?;

    res.render_msg("ok");

    Ok(())
}

#[handler]
pub async fn upload_avatar(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    todo!()
}
