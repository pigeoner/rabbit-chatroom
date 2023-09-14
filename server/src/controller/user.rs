mod handlers;

use self::handlers::*;
use super::auth::*;
use salvo::{prelude::JwtAuth, Router};

pub trait UserRoute {
    fn with_user_routes() -> Self;
    fn push_user_routes(self) -> Self;
}

impl UserRoute for Router {
    fn with_user_routes() -> Self {
        Router::new().push_user_routes()
    }
    fn push_user_routes(self) -> Self {
        self.push(Router::with_path("verifycode").get(get_verifycode))
            .push(
                Router::with_path("user")
                    .push(Router::with_path("signup").post(signup))
                    .push(Router::with_path("login").post(login))
                    .push(
                        Router::with_path("info")
                            .hoop_auth()
                            .get(get_self_userinfo)
                            .post(update_userinfo)
                            .push(Router::with_path("<userid:num>").get(get_userinfo))
                            .push(Router::with_path("avatar").post(upload_avatar)),
                    ),
            )
    }
}
