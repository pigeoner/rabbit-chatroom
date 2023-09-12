use sha256::digest;

use crate::user::types::UserLogin;

impl UserLogin {
    pub fn cal_avatar_file_name(&self) -> String {
        digest(&self.username)
    }
}
