use eframe::egui;
use jereide_core::{AppState, STATUS_BAR_BG, STATUS_BAR_MARGIN};

pub fn render_status_bar(state: &AppState, ui: &mut egui::Ui) {
    egui::Panel::bottom("status_bar")
        .frame(
            egui::Frame::NONE
                .fill(STATUS_BAR_BG)
                .inner_margin(STATUS_BAR_MARGIN),
        )
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Ready");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let tab = state.current_tab();
                    ui.label(format!("{}:{}", tab.cursor_line, tab.cursor_col));
                });
            });
        });
}
