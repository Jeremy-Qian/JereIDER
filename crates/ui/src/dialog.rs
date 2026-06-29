use eframe::egui;
use jereide_core::{AppState, SAVE_BUTTON_BG};

pub enum CloseConfirmAction {
    Save(usize),
    Discard(usize),
    Cancel,
}

pub fn render_close_confirm_modal(
    state: &mut AppState,
    ctx: &egui::Context,
) -> Option<CloseConfirmAction> {
    let idx = state.pending_close_index?;
    let file_name = state.tabs[idx].file_name();
    let dim_rect = ctx.viewport_rect();

    let dim_layer =
        egui::LayerId::new(egui::Order::Foreground, egui::Id::new("modal_dimmer"));
    let dim_painter = ctx.layer_painter(dim_layer);
    dim_painter.rect_filled(dim_rect, 0.0, egui::Color32::from_black_alpha(120));

    let response = egui::Window::new("Unsaved Changes")
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .order(egui::Order::Tooltip)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(format!(
                    "\"{}\" has unsaved changes.\nDo you want to save them before closing?",
                    file_name
                ));
            });

            ui.add_space(10.0);

            let btn_w = ui.available_width();
            let mut result: Option<CloseConfirmAction> = None;

            if ui
                .add_sized(
                    egui::vec2(btn_w, 0.0),
                    egui::Button::new("Save").fill(SAVE_BUTTON_BG),
                )
                .clicked()
            {
                result = Some(CloseConfirmAction::Save(idx));
            }

            if ui
                .add_sized(egui::vec2(btn_w, 0.0), egui::Button::new("Don't Save"))
                .clicked()
            {
                result = Some(CloseConfirmAction::Discard(idx));
            }

            if ui
                .add_sized(egui::vec2(btn_w, 0.0), egui::Button::new("Cancel"))
                .clicked()
            {
                result = Some(CloseConfirmAction::Cancel);
            }

            result
        });

    match response {
        Some(ir) => ir.inner.flatten(),
        None => None,
    }
}
