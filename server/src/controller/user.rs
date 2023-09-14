mod auth;
mod handlers;

use self::auth::*;
use self::handlers::*;
use salvo::{prelude::JwtAuth, Router};

pub trait UserController {
    fn push_user_routes(self) -> Self;
}

impl UserController for Router {
    fn push_user_routes(self) -> Self {
        let auth_handler: JwtAuth<JwtClaims, _> = get_auth_handler();

        self.push(Router::with_path("verifycode").get(get_verifycode))
            .push(
                Router::with_path("user")
                    .push(Router::with_path("signup").post(signup))
                    .push(Router::with_path("login").post(login))
                    .push(
                        Router::with_path("info")
                            .hoop(auth_handler)
                            .hoop(check_user_authed)
                            .get(get_self_userinfo)
                            .post(update_userinfo)
                            .push(Router::with_path("<userid:num>").get(get_userinfo))
                            .push(Router::with_path("avatar").post(upload_avatar)),
                    ),
            )
    }
}
