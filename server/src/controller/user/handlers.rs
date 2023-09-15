use anyhow::{anyhow, Result};
use jsonwebtoken::EncodingKey;
use salvo::prelude::*;

use super::RenderError;
use crate::controller::auth::JwtClaims;
use crate::controller::utils::{TokenResponse, UrlResponse};
use crate::service::avatar::{self, AvatarHandler};
use crate::{
    common::CONFIG,
    controller::utils::RenderMsg,
    service::{
        user::{UserError, UserHandler, UserLogin, UserSignup, Userinfo},
        verifycode::{gen_verifycode_base64, VerifycodeStatus},
    },
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
        Ok(VerifycodeStatus::Fail) => {
            res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "验证码错误");
        }
        Ok(VerifycodeStatus::Expired) => {
            res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "验证码已过期");
        }
        Ok(VerifycodeStatus::Success) => match uh.signup(&user_signup).await {
            Err(e) => e.render_error(res),
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
        Err(e) => e.render_error(res),
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

    match uh.update_userinfo(userid, new_info).await {
        Err(e) => res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
        Ok(_) => res.render_msg("ok"),
    };

    res.render_msg("ok");

    Ok(())
}

#[handler]
pub async fn upload_avatar(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let avatar_file = match req.file("avatar").await {
        Some(file) => file,
        None => {
            res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "无效的avatar");
            return Ok(());
        }
    };
    let userid = depot.get::<i32>("authed_userid").unwrap().to_owned();

    match AvatarHandler::save_avatar(avatar_file, userid) {
        Ok(path) => {
            res.render(Json(UrlResponse::new(path)));
        }
        Err(e) => {
            res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
        }
    };

    Ok(())
}
