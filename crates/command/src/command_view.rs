use eframe::egui;
use jereide_core::{COMMAND_VIEW_BG, COMMAND_VIEW_TEXT};

// Renders the whole command view.
pub fn render_command_view(ui: &mut egui::Ui) {
    let rect = ui.max_rect();
    ui.painter()
        .rect_filled(rect, 0, COMMAND_VIEW_BG);
    // TODO: Still needs implementation
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "Needs implementation",
        egui::FontId::proportional(18.0),
        COMMAND_VIEW_TEXT,
    );
}
