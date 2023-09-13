use captcha::{gen, Difficulty};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::types::UserSignup;
use crate::utils::get_redis_conn;
use anyhow::Result;

fn _gen_verifycode_base64() -> (String, String) {
    let cpt = gen(Difficulty::Easy);
    let cpt_base64 = "data:image/png;base64,".to_string() + &cpt.as_base64().unwrap();
    let code = cpt.chars_as_string().to_lowercase();

    (cpt_base64, code)
}

pub async fn gen_verifycode_base64() -> Result<VerirycodeResponse> {
    let (cpt_base64, code) = _gen_verifycode_base64();

    let uuid = Uuid::new_v4();

    let mut conn = get_redis_conn().await?;
    conn.set_ex(&uuid.to_string(), code, 60 * 5).await?;

    Ok(VerirycodeResponse::new(uuid.to_string(), cpt_base64))
}

pub async fn verify(uuid: &str, code: &str) -> Result<VerifyResult> {
    let mut conn = get_redis_conn().await?;
    let res: Option<String> = conn.get(uuid).await?;
    conn.del(uuid).await?;
    match res {
        Some(res) => {
            if res == code {
                Ok(VerifyResult::Success)
            } else {
                Ok(VerifyResult::Fail)
            }
        }
        None => Ok(VerifyResult::Expired),
    }
}

pub enum VerifyResult {
    Success,
    Fail,
    Expired,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerirycodeResponse {
    uuid: String,
    base64: String,
}

impl VerirycodeResponse {
    pub fn new(uuid: String, base64: String) -> Self {
        Self { uuid, base64 }
    }
}

impl UserSignup {
    pub async fn verify(&self) -> Result<VerifyResult> {
        verify(&self.uuid, &self.verifycode).await
    }
}
