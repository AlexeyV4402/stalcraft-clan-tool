use std::path::Path;
use anyhow::{Context, Result};
use image::DynamicImage;


pub fn load_screenshot(path: &Path) -> Result<DynamicImage> {
    Ok(image::open(path).with_context(|| {"Ошибка загрузки фото"})?)
}