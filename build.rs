fn main() {
    println!("cargo:rerun-if-changed=src/logo.png");
    println!("cargo::rustc-check-cfg=cfg(has_logo)");

    let logo_path = "src/logo.png";
    if !std::path::Path::new(logo_path).exists() {
        return;
    }

    println!("cargo:rustc-cfg=has_logo");

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        if generate_ico(logo_path) {
            let mut res = winresource::WindowsResource::new();
            res.set_icon("assets/BoxWrap.ico");
            res.compile().ok();
        }
    }
}

fn generate_ico(logo_path: &str) -> bool {
    let img = match image::open(logo_path) {
        Ok(img) => img,
        Err(_) => return false,
    };

    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    for &size in &[16u32, 32, 48, 256] {
        let resized = img.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
        let rgba = resized.to_rgba8();
        let icon_image = ico::IconImage::from_rgba_data(size, size, rgba.into_raw());
        match ico::IconDirEntry::encode(&icon_image) {
            Ok(entry) => icon_dir.add_entry(entry),
            Err(_) => return false,
        }
    }

    std::fs::create_dir_all("assets").ok();
    match std::fs::File::create("assets/BoxWrap.ico") {
        Ok(file) => icon_dir.write(file).is_ok(),
        Err(_) => false,
    }
}
