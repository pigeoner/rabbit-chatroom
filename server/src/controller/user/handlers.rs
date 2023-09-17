use anyhow::Result;
use salvo::prelude::*;

use super::utils::{render_error, RenderError, RenderMsg};
use crate::{
    controller::{
        auth::{set_cookie_token, AUTHED_USERID_KEY},
        utils::BadRequest,
    },
    entity::user::types::Username,
    service::{
        avatar::AvatarHandler,
        user::{UserHandler, UserLogin, UserSignup, UserUpdatePwd, Userinfo},
        verifycode::gen_verifycode_base64,
    },
};

#[handler]
pub async fn get_verifycode(res: &mut Response) -> Result<()> {
    let vr = render_error!(gen_verifycode_base64().await, res);
    res.render(Json(vr));
    Ok(())
}

#[handler]
pub async fn signup(req: &mut Request, res: &mut Response) -> Result<()> {
    let mut uh = render_error!(UserHandler::new().await, res);
    let user_signup: UserSignup = render_error!(req.parse_json().await, res);

    log::debug!("new signup: {:?}", user_signup);

    render_error!(user_signup.verify().await, res);
    render_error!(uh.signup(&user_signup).await, res);

    res.render_ok();

    Ok(())
}

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) -> Result<()> {
    let mut uh = render_error!(UserHandler::new().await, res);
    let user_login: UserLogin = render_error!(req.parse_json().await, res);

    log::debug!("new login: {:?}", user_login);

    let userid = render_error!(uh.login(&user_login).await, res);
    render_error!(set_cookie_token(res, userid).await, res);

    res.render_ok();

    Ok(())
}

#[handler]
pub async fn get_self_userinfo(depot: &mut Depot, res: &mut Response) -> Result<()> {
    let userid = depot.get::<i32>(AUTHED_USERID_KEY).unwrap().to_owned();

    let mut uh = render_error!(UserHandler::new().await, res);
    let user_info = render_error!(uh.get_userinfo(userid).await, res);

    res.render(Json(user_info));

    Ok(())
}

#[handler]
pub async fn get_userinfo(req: &mut Request, res: &mut Response) -> Result<()> {
    let userid: i32 = render_error!(
        req.param("userid").ok_or(anyhow::anyhow!("无效的userid")),
        res
    );

    let mut uh = render_error!(UserHandler::new().await, res);
    let user_info = render_error!(uh.get_userinfo(userid).await, res);

    res.render(Json(user_info));

    Ok(())
}

#[handler]
pub async fn get_userinfo_by_name(req: &mut Request, res: &mut Response) -> Result<()> {
    let username: Username = render_error!(req.parse_json().await, res);
    let username = username.username;

    let mut uh = render_error!(UserHandler::new().await, res);
    let user_info = render_error!(uh.get_userinfo_by_name(&username).await, res);
    res.render(Json(user_info));

    Ok(())
}

#[handler]
pub async fn update_userinfo(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    let userid = depot.get::<i32>(AUTHED_USERID_KEY).unwrap().to_owned();

    let new_info: Userinfo = render_error!(req.parse_json().await, res);
    let mut uh = render_error!(UserHandler::new().await, res);

    if userid != new_info.userid {
        res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "无效的userid");
        return Ok(());
    }

    render_error!(uh.update_userinfo(userid, new_info).await, res);

    res.render_ok();

    Ok(())
}

#[handler]
pub async fn update_password(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    let userid = depot.get::<i32>(AUTHED_USERID_KEY).unwrap().to_owned();

    let uup: UserUpdatePwd = render_error!(req.parse_json().await, res);

    if userid != uup.userid {
        res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "无效的userid");
        return Ok(());
    }

    let mut uh = render_error!(UserHandler::new().await, res);
    render_error!(uh.update_password(uup).await, res);

    res.render_ok();

    Ok(())
}

#[handler]
pub async fn upload_avatar(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let userid = depot.get::<i32>(AUTHED_USERID_KEY).unwrap().to_owned();

    let avatar_file = render_error!(
        req.file("avatar").await.ok_or(BadRequest("无效的avatar")),
        res
    );

    render_error!(AvatarHandler::save_avatar(avatar_file, userid).await, res);

    res.render_ok();

    Ok(())
}
