use sha2::Sha256;

use crate::types::User;

impl User {
    pub fn get_avatar_filename(&self) -> String {

        let mut hasher = Sha256::new();
    }
}
