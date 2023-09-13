mod jwt;
mod user;
mod utils;

use jwt::JwtClaims;
use jwt::get_auth_handler;

use salvo::prelude::*;

use crate::common::CONFIG;

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
                .hoop(auth_handler)
                .push(
                    Router::with_path("info")
                        .post(user::update_user_info)
                        .push(Router::with_path("<userId>").get(user::get_user_info))
                        .push(Router::with_path("avatar").post(user::upload_avatar)),
                ),
        );

    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn hello(res: &mut Response) {
    res.render("hello world");
}
