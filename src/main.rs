#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

fn setup_fonts(ctx: &eframe::egui::Context) {
    let mut fonts = eframe::egui::FontDefinitions::default();
    // 맑은 고딕: Windows 7+ 기본 한글 폰트
    if let Ok(data) = std::fs::read("C:\\Windows\\Fonts\\malgun.ttf") {
        fonts.font_data.insert(
            "malgun".to_owned(),
            std::sync::Arc::new(eframe::egui::FontData::from_owned(data)),
        );
        fonts
            .families
            .get_mut(&eframe::egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "malgun".to_owned());
    }
    ctx.set_fonts(fonts);
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "BoxWrap",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(app::BoxWrapApp::default()))
        }),
    )
}
