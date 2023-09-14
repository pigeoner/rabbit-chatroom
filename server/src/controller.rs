mod auth;
mod user;
mod utils;

use auth::get_auth_handler;
use auth::JwtClaims;

use salvo::prelude::*;

use crate::common::CONFIG;

use self::auth::check_user_authed;

pub async fn serve() {
    let auth_handler: JwtAuth<JwtClaims, _> = get_auth_handler();

    let acceptor = TcpListener::new(&CONFIG.addr_listen).bind().await;

    let router = Router::with_path("chatroom/v1")
        .get(hello)
        .push(Router::with_path("verifycode").get(user::get_verifycode))
        .push(
            Router::with_path("user")
                .push(Router::with_path("signup").post(user::signup))
                .push(Router::with_path("login").post(user::login))
                .push(
                    Router::with_path("info")
                        .hoop(auth_handler)
                        .hoop(check_user_authed)
                        .get(user::get_self_userinfo)
                        .post(user::update_userinfo)
                        .push(Router::with_path("<userid:num>").get(user::get_userinfo))
                        .push(Router::with_path("avatar").post(user::upload_avatar)),
                ),
        );

    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn hello(res: &mut Response) {
    res.render("hello world");
}