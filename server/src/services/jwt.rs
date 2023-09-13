use salvo::{prelude::JwtAuth, jwt_auth::{HeaderFinder, ConstDecoder}};
use serde::{Deserialize, Serialize};

use crate::common::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub username: String,
    pub exp: i64,
}

pub fn get_auth_handler() -> JwtAuth<JwtClaims, ConstDecoder> {
    JwtAuth::new(ConstDecoder::from_secret(&CONFIG.jwt_secret.as_bytes()))
        .finders(vec![Box::new(HeaderFinder::new())])
        .force_passed(true)
}