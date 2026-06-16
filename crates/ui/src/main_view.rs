use eframe::egui;
use jereide_core::{char_index_to_line_col, AppState};
use jereide_syntax::SyntaxHighlighter;

use crate::title_bar;

pub fn render_central_panel(state: &mut AppState, ctx: &egui::Context) {
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(egui::Color32::WHITE))
        .show(ctx, |ui| {
            let style = ui.style_mut();
            style.visuals.extreme_bg_color = egui::Color32::WHITE;
            style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
            style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
            style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
            style.spacing.scroll = {
                let mut s = egui::style::ScrollStyle::solid();
                s.bar_width = 12.0;
                s
            };

            let is_fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            title_bar::render_title_bar(state, ui, is_fullscreen);

            let highlighter = SyntaxHighlighter::new(14.0);

            let mut layouter = |ui: &egui::Ui, text: &str, _max_width: f32| {
                let layout_job = highlighter.highlight(text);
                ui.fonts(|f| f.layout_job(layout_job))
            };

            let editor_available = ui.available_size();

            let output = egui::ScrollArea::both()
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.add_sized(
                        editor_available,
                        egui::TextEdit::code_editor(egui::TextEdit::multiline(&mut state.code_text))
                            .id_source("editor")
                            .frame(false)
                            .margin(5)
                            .layouter(&mut layouter),
                    )
                });

            let response = output.inner;
            state.editor_id = response.id;

            if let Some(edit_state) = egui::TextEdit::load_state(ctx, response.id) {
                if let Some(range) = edit_state.cursor.char_range() {
                    let (line, col) =
                        char_index_to_line_col(&state.code_text, range.primary.index);
                    state.cursor_line = line;
                    state.cursor_col = col;
                }
            }

            if !state.editor_focused {
                state.editor_focused = true;
                response.request_focus();
            }
        });
}
