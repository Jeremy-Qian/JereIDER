use eframe::egui;
use crate::syntax::SyntaxHighlighter;
use crate::JereIDEApp;

impl JereIDEApp {
    pub fn render_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(egui::Color32::WHITE))
            .show(ctx, |ui| {
                let available = ui.available_size();

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

                let highlighter = SyntaxHighlighter::new(14.0);

                let mut layouter = |ui: &egui::Ui, text: &str, _max_width: f32| {
                    let layout_job = highlighter.highlight(text);
                    ui.fonts(|f| f.layout_job(layout_job))
                };

                let output = egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        ui.add_sized(
                            available,
                            egui::TextEdit::code_editor(egui::TextEdit::multiline(&mut self.code_text))
                                .id_source("editor")
                                .frame(false)
                                .margin(5)
                                .layouter(&mut layouter),
                        )
                    });

                let response = output.inner;
                self.editor_id = response.id;

                if let Some(state) = egui::TextEdit::load_state(ctx, response.id) {
                    if let Some(range) = state.cursor.char_range() {
                        let (line, col) =
                            char_index_to_line_col(&self.code_text, range.primary.index);
                        self.cursor_line = line + 1;
                        self.cursor_col = col + 1;
                    }
                }

                if !self.editor_focused {
                    self.editor_focused = true;
                    response.request_focus();
                }
            });
    }
}

fn char_index_to_line_col(text: &str, char_index: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;
    for (ci, ch) in text.chars().enumerate() {
        if ci >= char_index {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}
