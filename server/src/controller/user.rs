mod handlers;

use salvo::prelude::*;

use crate::service::{avatar::AvatarError, user::UserError, verifycode::VerifycodeError};

use super::{
    auth::{self, HoopAuth},
    utils::{self, ErrorRender, RenderMsg},
};

use self::handlers::*;

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

impl ErrorRender for UserError {
    fn error_render(&self, res: &mut Response) {
        match self {
            UserError::UsernameAlreadyExists => {
                res.render_statuscoded_msg(StatusCode::CONFLICT, "用户名已存在");
            }
            UserError::UserNotFound => {
                res.render_statuscoded_msg(StatusCode::NOT_FOUND, "用户不存在");
            }
            UserError::PasswordNotMatch => {
                res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "密码错误");
            }

            _ => {
                res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, &self.to_string());
            }
        }
    }
}

impl ErrorRender for AvatarError {
    fn error_render(&self, res: &mut Response) {
        match self {
            AvatarError::WidthNotEqualHeight => {
                res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "图片宽高不相等");
            }
            AvatarError::UnacceptedImageFormat => {
                res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "不支持的图片格式");
            }

            _ => {
                res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, &self.to_string());
            }
        }
    }
}

impl ErrorRender for VerifycodeError {
    fn error_render(&self, res: &mut Response) {
        match self {
            VerifycodeError::Wrong => {
                res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "验证码错误");
            }
            VerifycodeError::Expired => {
                res.render_statuscoded_msg(StatusCode::BAD_REQUEST, "验证码已过期");
            }
            _ => {
                res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, &self.to_string());
            }
        }
    }
}
