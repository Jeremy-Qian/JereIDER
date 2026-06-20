use eframe::egui;
use jereide_core::AppState;

/// Renders the command palette / command view placeholder.
pub fn render_command_view(_state: &mut AppState, ui: &mut egui::Ui) {
    let available = ui.available_size();
    let (rect, _) = ui.allocate_exact_size(available, egui::Sense::hover());

    let text_pos = egui::pos2(
        rect.center().x,
        rect.center().y,
    );

    ui.painter().text(
        egui::Pos2::new(text_pos.x, text_pos.y),
        egui::Align2::CENTER_CENTER,
        "Needs implementation",
        egui::FontId::proportional(18.0),
        egui::Color32::from_gray(160),
    );
}
