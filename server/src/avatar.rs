use sha256::digest;

use crate::types::User;

impl User {
    pub fn get_avatar_file_name(&self) -> String {
        digest(&self.username)
    }
}
