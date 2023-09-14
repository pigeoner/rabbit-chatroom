use salvo::{prelude::StatusCode, writing::Json, Response};
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResponse {
    pub token: String,
}

impl TokenResponse {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UrlResponse {
    pub url: String,
}

impl UrlResponse {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}
