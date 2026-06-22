use jereide_menu::AppMenu;

// Entry point to JereIDE. Initiated with 800x600 size,
// no title bar, no title, and a full-size content view.

fn main() -> Result<(), eframe::Error> {
    let app_menu = AppMenu::new();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_titlebar_shown(false)
            .with_title_shown(false)
            .with_fullsize_content_view(true),
        ..Default::default()
    };

    eframe::run_native(
        "jereide",
        options,
        Box::new(|_cc| Ok(Box::new(jereide_main::JereIDEApp::new(app_menu)))),
    )
}
