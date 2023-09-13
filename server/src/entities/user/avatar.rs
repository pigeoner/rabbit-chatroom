use sha256::digest;

use super::types::UserLogin;

impl UserLogin {
    pub fn cal_avatar_file_name(&self) -> String {
        digest(&self.username)
    }
}
