use salvo::http::form::FilePart;

use anyhow::Result;

use crate::{common::CONFIG, entity::image::save_image};

type Path = String;

pub async fn save_avatar(file: &FilePart, userid: i32) -> Result<Path> {
    let dest = format!("{}{}.png", CONFIG.image_dir, userid);

    save_image(file, &dest);

    Ok(dest)
}
