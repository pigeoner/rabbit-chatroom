use salvo::http::form::FilePart;

use anyhow::Result;

use crate::{common::CONFIG, entity::image::save_avatar};

type Url = String;

pub struct AvatarHandler {}

impl AvatarHandler {
    pub fn save_avatar(file: &FilePart, userid: i32) -> AvatarResult<Url> {
        let img = image::open(file.path()).map_err(|_| AvatarError::UnacceptedImageFormat)?;
    
        let (width, height) = (img.width(), img.height());
        if width != height {
            return Err(AvatarError::WidthNotEqualHeight);
        }
        
        let userid = Userid(userid);
        save_avatar(img, userid.to_path())?;
    
        Ok(userid.to_url())
    }
    
}

struct Userid(i32);

impl Userid {
    fn to_path(&self) -> String {
        format!("{}{}.png", CONFIG.avatar_dir, self.0)
    }

    fn to_url(&self) -> String {
        format!("{}{}.png", CONFIG.image_url, self.to_path())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AvatarError {
    #[error("不被接受的图片格式")]
    UnacceptedImageFormat,
    #[error("长宽不相等")]
    WidthNotEqualHeight,
    #[error("其他错误：{0}")]
    Other(anyhow::Error),
} 

pub type AvatarResult<T> = Result<T, AvatarError>;

impl From<anyhow::Error> for AvatarError {
    fn from(e: anyhow::Error) -> Self {
        Self::Other(e)
    }
}