use std::fmt::{self, Display, Formatter};

use salvo::{http::ParseError, prelude::StatusCode, writing::Json, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MsgResponse {
    pub msg: String,
}

impl MsgResponse {
    pub fn new(msg: &str) -> MsgResponse {
        MsgResponse {
            msg: msg.to_string(),
        }
    }

    pub fn json(msg: &str) -> Json<MsgResponse> {
        Json(MsgResponse::new(msg))
    }
}

pub trait RenderMsg {
    fn render_msg(&mut self, msg: &str);
    fn render_statuscoded_msg(&mut self, status_code: StatusCode, msg: &str);
    fn render_ok(&mut self) {
        self.render_msg("ok");
    }
}

impl RenderMsg for Response {
    fn render_msg(&mut self, msg: &str) {
        self.render(MsgResponse::json(msg));
    }
    fn render_statuscoded_msg(&mut self, status_code: StatusCode, msg: &str) {
        self.status_code(status_code);
        self.render_msg(msg);
    }
}

// 错误类型的特征，用于统一错误向客户端的render
pub trait ErrorRender {
    fn error_render(&self, res: &mut Response);
}

impl ErrorRender for anyhow::Error {
    fn error_render(&self, res: &mut Response) {
        res.render_statuscoded_msg(StatusCode::INTERNAL_SERVER_ERROR, &self.to_string());
    }
}

impl ErrorRender for ParseError {
    fn error_render(&self, res: &mut Response) {
        res.render_statuscoded_msg(StatusCode::BAD_REQUEST, &self.to_string());
    }
}

pub trait RenderError {
    fn render_error<E: ErrorRender>(&mut self, e: E);
}
impl RenderError for Response {
    fn render_error<E: ErrorRender>(&mut self, e: E) {
        e.error_render(self);
    }
}

#[derive(thiserror::Error, Debug)]
pub struct BadRequest(pub &'static str);

impl Display for BadRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ErrorRender for BadRequest {
    fn error_render(&self, res: &mut Response) {
        res.render_statuscoded_msg(StatusCode::BAD_REQUEST, &self.0);
    }
}

// 用于简化对Result进行match并render错误的宏
#[macro_export]
macro_rules! render_error {
    ($expr:expr, $res:expr) => {
        match $expr {
            Ok(value) => value,
            Err(e) => {
                $res.render_error(e);
                return Ok(());
            }
        }
    };
}
pub use render_error;

#[macro_export]
macro_rules! render_error_skip {
    ($expr:expr, $res:expr, $ctrl:expr) => {
        match $expr {
            Ok(value) => value,
            Err(e) => {
                $res.render_error(e);
                $ctrl.skip_rest();
                return Ok(());
            }
        }
    };
}
pub use render_error_skip;

// #[macro_export]
// macro_rules! render_ok {
//     ($res:expr) => {
//         $res.render_ok();
//         return Ok(())
//     };
// }
// pub use render_ok;
