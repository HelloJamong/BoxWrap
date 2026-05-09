use boxwrap::polyglot;
use eframe::egui;
use std::fs;
use std::path::PathBuf;

pub struct BoxWrapApp {
    image_path: Option<PathBuf>,
    zip_path: Option<PathBuf>,
    status: String,
}

impl Default for BoxWrapApp {
    fn default() -> Self {
        Self {
            image_path: None,
            zip_path: None,
            status: String::from("대기 중"),
        }
    }
}

impl eframe::App for BoxWrapApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("BoxWrap");
            ui.add_space(16.0);

            ui.label("포장지 이미지 (PNG/JPG)");
            ui.horizontal(|ui| {
                if ui.button("파일 선택...").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Images", &["png", "jpg", "jpeg"])
                        .pick_file()
                    {
                        self.image_path = Some(path);
                        self.status = String::from("대기 중");
                    }
                }
                let label = self
                    .image_path
                    .as_ref()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_else(|| String::from("선택되지 않음"));
                ui.label(label);
            });

            ui.add_space(8.0);

            ui.label("압축 파일 (ZIP)");
            ui.horizontal(|ui| {
                if ui.button("파일 선택...").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("ZIP", &["zip"])
                        .pick_file()
                    {
                        self.zip_path = Some(path);
                        self.status = String::from("대기 중");
                    }
                }
                let label = self
                    .zip_path
                    .as_ref()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_else(|| String::from("선택되지 않음"));
                ui.label(label);
            });

            ui.add_space(16.0);

            let can_wrap = self.image_path.is_some() && self.zip_path.is_some();
            ui.add_enabled_ui(can_wrap, |ui| {
                if ui.button("  포장하기  ").clicked() {
                    self.do_wrap();
                }
            });

            ui.add_space(8.0);
            ui.label(&self.status);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label(format!("v{}", env!("CARGO_PKG_VERSION")));
            });
        });
    }
}

impl BoxWrapApp {
    fn do_wrap(&mut self) {
        let image_path = match &self.image_path {
            Some(p) => p.clone(),
            None => return,
        };
        let zip_path = match &self.zip_path {
            Some(p) => p.clone(),
            None => return,
        };

        let result = (|| -> Result<PathBuf, String> {
            let image_bytes = fs::read(&image_path).map_err(|e| e.to_string())?;
            let zip_bytes = fs::read(&zip_path).map_err(|e| e.to_string())?;
            let output = polyglot::wrap(&image_bytes, &zip_bytes).map_err(|e| e.to_string())?;

            let ext = image_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("png");
            let default_name = format!("output.{ext}");

            let save_path = rfd::FileDialog::new()
                .set_file_name(&default_name)
                .add_filter("Image", &[ext])
                .save_file()
                .ok_or_else(|| String::from("저장 취소됨"))?;

            fs::write(&save_path, &output).map_err(|e| e.to_string())?;
            Ok(save_path)
        })();

        self.status = match result {
            Ok(path) => format!("포장 완료: {}", path.display()),
            Err(e) => format!("오류: {e}"),
        };
    }
}
