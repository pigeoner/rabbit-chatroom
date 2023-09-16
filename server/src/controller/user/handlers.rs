use anyhow::Result;
use salvo::prelude::*;

use super::utils::{render_error, RenderError, RenderMsg, TokenResponse};
use crate::{
    controller::{auth::sign_token, utils::BadRequest},
    entity::user::types::Username,
    service::{
        avatar::AvatarHandler,
        user::{UserHandler, UserLogin, UserSignup, UserUpdatePwd, Userinfo},
        verifycode::gen_verifycode_base64,
    },
};

#[handler]
pub async fn get_verifycode(res: &mut Response) -> Result<()> {
    let vr = render_error!(res, gen_verifycode_base64().await);
    res.render(Json(vr));
    Ok(())
}

#[handler]
pub async fn signup(req: &mut Request, res: &mut Response) -> Result<()> {
    let mut uh = render_error!(res, UserHandler::new().await);
    let user_signup: UserSignup = render_error!(res, req.parse_json().await);

    log::debug!("new signup: {:?}", user_signup);

    render_error!(res, user_signup.verify().await);
    render_error!(res, uh.signup(&user_signup).await);

    res.render_ok();

    Ok(())
}

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) -> Result<()> {
    let mut uh = render_error!(res, UserHandler::new().await);
    let user_login: UserLogin = render_error!(res, req.parse_json().await);

    log::debug!("new login: {:?}", user_login);

    let userid = render_error!(res, uh.login(&user_login).await);
    let token = render_error!(res, sign_token(userid).await);

    res.render(Json(TokenResponse::new(token)));

    Ok(())
}

#[handler]
pub async fn get_self_userinfo(depot: &mut Depot, res: &mut Response) -> Result<()> {
    let userid = depot.get::<i32>("authed_userid").unwrap().to_owned();

    let mut uh = render_error!(res, UserHandler::new().await);
    let user_info = render_error!(res, uh.get_userinfo(userid).await);

    res.render(Json(user_info));

    Ok(())
}

#[handler]
pub async fn get_userinfo(req: &mut Request, res: &mut Response) -> Result<()> {
    let userid: i32 = render_error!(
        res,
        req.param("userid").ok_or(anyhow::anyhow!("无效的userid"))
    );

    let mut uh = render_error!(res, UserHandler::new().await);
    let user_info = render_error!(res, uh.get_userinfo(userid).await);

    res.render(Json(user_info));

    Ok(())
}

#[handler]
pub async fn get_userinfo_by_name(req: &mut Request, res: &mut Response) -> Result<()> {
    let username: Username = render_error!(res, req.parse_json().await);
    let username = username.username;

    let mut uh = render_error!(res, UserHandler::new().await);
    let user_info = render_error!(res, uh.get_userinfo_by_name(&username).await);
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

    let new_info: Userinfo = render_error!(res, req.parse_json().await);
    let mut uh = render_error!(res, UserHandler::new().await);

    if userid != new_info.userid {
        res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "无效的userid");
        return Ok(());
    }

    render_error!(res, uh.update_userinfo(userid, new_info).await);

    res.render_ok();

    Ok(())
}

#[handler]
pub async fn update_password(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    let userid = depot.get::<i32>("authed_userid").unwrap().to_owned();

    let uup: UserUpdatePwd = render_error!(res, req.parse_json().await);

    if userid != uup.userid {
        res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "无效的userid");
        return Ok(());
    }

    let mut uh = render_error!(res, UserHandler::new().await);
    render_error!(res, uh.update_password(uup).await);

    res.render_ok();

    Ok(())
}

#[handler]
pub async fn upload_avatar(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let userid = depot.get::<i32>("authed_userid").unwrap().to_owned();

    let avatar_file = render_error!(
        res,
        req.file("avatar").await.ok_or(BadRequest("无效的avatar"))
    );

    render_error!(res, AvatarHandler::save_avatar(avatar_file, userid).await);

    res.render_ok();

    Ok(())
}
