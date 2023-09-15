use anyhow::Result;
use image::DynamicImage;

use anyhow::anyhow;

use crate::common::CONFIG;

pub fn save_avatar(image: DynamicImage, path: String) -> Result<()> {
    let dest = path_to_dest(path);

    image.save_with_format(dest, image::ImageFormat::Png)
        .map_err(|e| anyhow!("save image failed: {}", e))
}

pub fn path_to_dest(path: String) -> String {
    format!("{}{}", CONFIG.image_dir, path)
}