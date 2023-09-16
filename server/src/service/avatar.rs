use image::{codecs::jpeg::JpegEncoder, DynamicImage};
use salvo::http::form::FilePart;

use anyhow::Result;
use tokio::task::spawn_blocking;

use crate::{common::CONFIG, entity::image::save_image_data};

pub struct AvatarHandler {}

impl AvatarHandler {
    pub async fn save_avatar(file: &FilePart, userid: i32) -> AvatarResult<()> {
        let img = image::open(file.path()).map_err(|_| AvatarError::UnacceptedImageFormat)?;

        let (width, height) = (img.width(), img.height());
        if width != height {
            return Err(AvatarError::WidthNotEqualHeight);
        }

        let _img = img.clone();
        let handler = spawn_blocking(move || {
            let img = resize_avatar(&img);
            encode_avatar(&img)
        });
        let handler_mini = spawn_blocking(move || {
            let img = resize_mini_avatar(&_img);
            encode_avatar(&img)
        });

        let encoded_data = handler.await.map_err(anyhow::Error::from)??;
        let encoded_data_mini = handler_mini.await.map_err(anyhow::Error::from)??;

        let userid = Userid(userid);

        let future1 = save_image_data(encoded_data, userid.path_under_img());
        let future2 = save_image_data(encoded_data_mini, userid.path_mini_under_img());

        tokio::try_join!(future1, future2)?;

        Ok(())
    }
}

struct Userid(i32);

impl Userid {
    fn path_under_img(&self) -> String {
        format!("{}{}.jpg", CONFIG.avatar_dir, self.0)
    }

    fn path_mini_under_img(&self) -> String {
        format!("{}mini/{}.jpg", CONFIG.avatar_dir, self.0)
    }
}

pub type AvatarResult<T> = Result<T, AvatarError>;

#[derive(thiserror::Error, Debug)]
pub enum AvatarError {
    #[error("不被接受的图片格式")]
    UnacceptedImageFormat,
    #[error("长宽不相等")]
    WidthNotEqualHeight,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

fn encode_avatar(img: &DynamicImage) -> Result<Vec<u8>> {
    let mut encoded_data = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut encoded_data, CONFIG.avatar_quality);
    encoder.encode_image(img)?;
    Ok(encoded_data)
}

fn resize_avatar(img: &DynamicImage) -> DynamicImage {
    img.resize_exact(
        CONFIG.avatar_width,
        CONFIG.avatar_width,
        image::imageops::FilterType::Lanczos3,
    )
}

fn resize_mini_avatar(img: &DynamicImage) -> DynamicImage {
    img.resize_exact(
        CONFIG.avatar_mini_width,
        CONFIG.avatar_mini_width,
        image::imageops::FilterType::Lanczos3,
    )
}
