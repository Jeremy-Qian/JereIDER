use eframe::egui;
use jereide_core::{char_index_to_line_col, AppState};
use jereide_syntax::SyntaxHighlighter;

/// Renders the code editor view — a syntax-highlighted multi-line text editor.
pub fn render_code_view(state: &mut AppState, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();

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

    let extension = state
        .current_file_path
        .as_ref()
        .and_then(|p| std::path::Path::new(p).extension())
        .and_then(|ext| ext.to_str());

    let highlighter = SyntaxHighlighter::new(14.0, extension);

    let mut layouter =
        |ui: &egui::Ui, text: &dyn egui::widgets::TextBuffer, _max_width: f32| {
            let layout_job = highlighter.highlight(text.as_str());
            ui.fonts_mut(|f| f.layout_job(layout_job))
        };

    let response = egui::ScrollArea::both()
        .auto_shrink(false)
        .show(ui, |ui| {
            ui.add(
                egui::TextEdit::code_editor(egui::TextEdit::multiline(&mut state.code_text))
                    .id_source("editor")
                    .frame(egui::Frame {
                        inner_margin: egui::Margin::same(10),
                        ..egui::Frame::NONE
                    })
                    .layouter(&mut layouter),
            )
        })
        .inner;
    state.editor_id = response.id;

    if let Some(edit_state) = egui::TextEdit::load_state(&ctx, response.id) {
        if let Some(range) = edit_state.cursor.char_range() {
            let (line, col) = char_index_to_line_col(&state.code_text, range.primary.index);
            state.cursor_line = line;
            state.cursor_col = col;
        }
    }

    if !state.editor_focused {
        state.editor_focused = true;
        response.request_focus();
    }
}

