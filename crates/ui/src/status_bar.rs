use eframe::egui;
use jereide_core::AppState;

pub fn render_status_bar(state: &AppState, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("status_bar")
        .frame(
            egui::Frame::NONE
                .fill(egui::Color32::WHITE)
                .inner_margin(egui::Margin::symmetric(8, 4)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Ready");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Ln {}, Col {}", state.cursor_line, state.cursor_col));
                });
            });
        });
}
