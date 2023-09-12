use sha256::digest;

use crate::user_type::UserLogin;

impl UserLogin {
    pub fn get_avatar_file_name(&self) -> String {
        digest(&self.username)
    }
}
