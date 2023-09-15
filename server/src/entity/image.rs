use std::io::Cursor;

use anyhow::{anyhow, Result};

use image::DynamicImage;

use crate::common::CONFIG;

pub async fn save_image(image: DynamicImage, path_under_img: String) -> Result<()> {
    let data = image_to_data(image)?;

    save_image_data(data, path_under_img).await
}

pub async fn save_image_data(image_data: impl AsRef<[u8]>, path_under_img: String) -> Result<()> {
    let dest = path_to_dest(path_under_img);

    save_file(dest, image_data.as_ref()).await?;

    Ok(())
}

pub fn path_to_dest(path: String) -> String {
    format!("{}{}{}", CONFIG.static_dir, CONFIG.image_dir, path)
}

fn image_to_data(img: DynamicImage) -> Result<Vec<u8>> {
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, image::ImageFormat::Png)
        .map_err(|_| anyhow!("write image to buffer failed"))?;
    let data = buffer.into_inner();

    Ok(data)
}

async fn save_file(dest: String, contents: &[u8]) -> Result<()> {
    tokio::fs::write(dest, contents).await?;

    Ok(())
}
