use eframe::egui;
use jereide_core::AppState;

/// Renders the command palette / command view placeholder.
pub fn render_command_view(_state: &mut AppState, ui: &mut egui::Ui) {
    let rect = ui.max_rect();
    ui.painter().rect_filled(rect, 0, egui::Color32::from_gray(20));

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "Needs implementation",
        egui::FontId::proportional(18.0),
        egui::Color32::from_gray(250),
    );
}
