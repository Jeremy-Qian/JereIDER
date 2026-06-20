use eframe::egui;
use jereide_core::{AppState, CurrentView};

pub fn render_title_bar(state: &mut AppState, ui: &mut egui::Ui, is_fullscreen: bool) {
    let available = ui.available_size();
    let gray_bar_height = 34.0;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(available.x, gray_bar_height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgb(245, 245, 245));
    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.style_mut().text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::proportional(12.0),
        );

        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            if is_fullscreen {
                ui.add_space(7.0);
            } else {
                ui.add_space(75.0); // For traffic lights
            }

            if ui
                .selectable_label(state.current_view == CurrentView::Code, "Code")
                .clicked()
            {
                state.slide_to_view(CurrentView::Code);
            }
            if ui
                .selectable_label(state.current_view == CurrentView::Command, "Command")
                .clicked()
            {
                state.slide_to_view(CurrentView::Command);
            }
        });
    });
}
