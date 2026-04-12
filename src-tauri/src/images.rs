use base64::Engine;
use rand::seq::SliceRandom;
use std::fs;
use std::path::Path;

const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "bmp"];

pub struct ImageResult {
    pub data_url: String,
    pub path: String,
}

pub fn get_random_image_base64(images_path: &str) -> Option<ImageResult> {
    if images_path.is_empty() {
        return None;
    }

    let dir = Path::new(images_path);
    if !dir.is_dir() {
        return None;
    }

    let entries: Vec<_> = fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
                .unwrap_or(false)
        })
        .collect();

    if entries.is_empty() {
        return None;
    }

    let mut rng = rand::thread_rng();
    let entry = entries.choose(&mut rng)?;
    let path = entry.path();

    let bytes = fs::read(&path).ok()?;
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png")
        .to_lowercase();

    let mime = match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        _ => "image/png",
    };

    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Some(ImageResult {
        data_url: format!("data:{};base64,{}", mime, b64),
        path: path.to_string_lossy().to_string(),
    })
}
