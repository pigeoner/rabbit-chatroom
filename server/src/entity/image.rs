use anyhow::Result;
use salvo::http::form::FilePart;
use tokio::fs;

use anyhow::anyhow;

use crate::common::CONFIG;

pub async fn save_image(file: &FilePart, dest: &str) -> Result<()> {
    let dest = format!("{}{}", CONFIG.image_dir, dest);

    fs::copy(file.path(), dest).await?;

    Ok(())
}