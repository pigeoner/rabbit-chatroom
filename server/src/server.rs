use anyhow::{anyhow, Result};
use jsonwebtoken::EncodingKey;
use salvo::{
    http::cookie::time::{Duration, OffsetDateTime},
    jwt_auth::{ConstDecoder, HeaderFinder},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::CONFIG,
    user::types::{UserError, UserLogin, UserSignup},
    user::UserHandler,
    verifycode::gen_register_verifycode,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    username: String,
    exp: i64,
}

pub async fn serve() {
    let auth_handler: JwtAuth<JwtClaims, _> =
        JwtAuth::new(ConstDecoder::from_secret(&CONFIG.jwt_secret.as_bytes()))
            .finders(vec![Box::new(HeaderFinder::new())])
            .force_passed(true);

    let acceptor = TcpListener::new("127.0.0.1:80").bind().await;

    let router = Router::with_path("chatroom/v1")
        .get(hello)
        .push(Router::with_path("verifycode").get(get_verifycode))
        .push(
            Router::with_path("user")
                .push(Router::with_path("signup").post(signup))
                .push(Router::with_path("login").post(login))
                .hoop(auth_handler)
                .push(
                    Router::with_path("info")
                        .post(update_user_info)
                        .push(Router::with_path("<userId>").get(get_user_info))
                        .push(Router::with_path("avatar").post(upload_avatar)),
                ),
        );

    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn hello(res: &mut Response) {
    res.render("hello world");
}

#[handler]
async fn get_verifycode(res: &mut Response) -> Result<()> {
    let vr = gen_register_verifycode().await?;
    res.render(Json(vr));
    Ok(())
}

#[handler]
async fn signup(req: &mut Request, res: &mut Response) -> Result<()> {
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
async fn login(req: &mut Request, res: &mut Response) -> Result<()> {
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
async fn get_user_info(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    todo!()
}

#[handler]
async fn update_user_info(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    todo!()
}

#[handler]
async fn upload_avatar(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    todo!()
}
