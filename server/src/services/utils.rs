use salvo::{prelude::StatusCode, writing::Json, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Msg {
    pub msg: String,
}

impl Msg {
    pub fn new(msg: &str) -> Msg {
        Msg {
            msg: msg.to_string(),
        }
    }

    pub fn json(msg: &str) -> Json<Msg> {
        Json(Msg::new(msg))
    }
}

pub trait RenderMsg {
    fn render_msg(&mut self, msg: &str);
    fn render_statuscoded_msg(&mut self, status_code: StatusCode, msg: &str);
}

impl RenderMsg for Response {
    fn render_msg(&mut self, msg: &str) {
        self.render(Msg::json(msg));
    }
    fn render_statuscoded_msg(&mut self, status_code: StatusCode, msg: &str) {
        self.status_code(status_code);
        self.render_msg(msg);
    }
}
