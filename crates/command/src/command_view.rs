use eframe::egui;

// Renders the whole command view.
pub fn render_command_view(ui: &mut egui::Ui) {
    let rect = ui.max_rect();
    ui.painter()
        .rect_filled(rect, 0, egui::Color32::from_gray(20));
    // TODO: Still needs implementation
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "Needs implementation",
        egui::FontId::proportional(18.0),
        egui::Color32::from_gray(250),
    );
}
