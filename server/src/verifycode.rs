use std::sync::Arc;

use captcha::{gen, Difficulty};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

use crate::{user::types::UserSignup, utils::get_redis_conn};
use anyhow::Result;

fn gen_verifycode_base64() -> (String, String) {
    let cpt = gen(Difficulty::Easy);
    let cpt_base64 = cpt.as_base64().unwrap();
    let code = cpt.chars_as_string();

    (cpt_base64, code)
}

pub async fn gen_register_verifycode() -> Result<VerirycodeResponse> {
    let (cpt_base64, code) = gen_verifycode_base64();

    let uuid = Uuid::new_v4();

    let mut conn = get_redis_conn().await?;
    conn.set_ex(&uuid.to_string(), code, 60 * 5).await?;

    Ok(VerirycodeResponse::new(uuid.to_string(), cpt_base64))
}

pub async fn verify(uuid: &str, code: &str) -> Result<bool> {
    let mut conn = get_redis_conn().await?;
    let res: Option<String> = conn.get(uuid).await?;
    match res {
        Some(res) => Ok(res == code),
        None => Ok(false),
    }
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
    pub async fn verify(&self) -> Result<bool> {
        let mut conn = get_redis_conn().await?;
        let res: Option<String> = conn.get(&self.uuid).await?;
        match res {
            Some(res) => Ok(res == self.verifycode),
            None => Ok(false),
        }
    }
}