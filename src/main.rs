use eframe::egui;

mod menu;

use menu::AppMenu;

fn main() -> Result<(), eframe::Error> {
    let app_menu = AppMenu::new();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("JereIDE"),
        ..Default::default()
    };

    eframe::run_native(
        "JereIDE",
        options,
        Box::new(|_cc| Ok(Box::new(EditorApp::new(app_menu)))),
    )
}

struct EditorApp {
    code_text: String,
    editor_focused: bool,
    cursor_line: usize,
    cursor_col: usize,
    app_menu: AppMenu,
    editor_id: egui::Id,
}

impl EditorApp {
    fn new(app_menu: AppMenu) -> Self {
        Self {
            code_text: String::new(),
            editor_focused: false,
            cursor_line: 1,
            cursor_col: 1,
            app_menu,
            editor_id: egui::Id::new("editor"),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.app_menu.is_initialized() {
            self.app_menu.init();
            self.app_menu.set_initialized();
        }

        for event_id in self.app_menu.poll_events() {
            match event_id.as_ref() {
                "new" | "open" | "save" => {
                    // File menu: not yet implemented
                }
                "quit" => std::process::exit(0),
                "about" => {}
                _ => self.handle_edit_action(ctx, event_id.as_ref()),
            }
        }

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
                        ui.label(format!("Ln {}, Col {}", self.cursor_line, self.cursor_col));
                    });
                });
            });

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

                let output = egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        ui.add_sized(
                            available,
                            egui::TextEdit::multiline(&mut self.code_text)
                                .id_source("editor")
                                .font(egui::TextStyle::Monospace)
                                .frame(false)
                                .margin(5)
                                .text_color(egui::Color32::BLACK),
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

impl EditorApp {
    fn handle_edit_action(&mut self, ctx: &egui::Context, action: &str) {
        match action {
            "select_all" => self.action_select_all(ctx),
            "copy" => self.action_copy(ctx),
            "cut" => self.action_cut(ctx),
            "paste" => self.action_paste(ctx),
            "undo" => self.action_undo(ctx),
            "redo" => self.action_redo(ctx),
            _ => {}
        }
    }

    fn action_select_all(&self, ctx: &egui::Context) {
        if let Some(mut state) = egui::TextEdit::load_state(ctx, self.editor_id) {
            let len = self.code_text.chars().count();
            use egui::text::{CCursor, CCursorRange};
            state
                .cursor
                .set_char_range(Some(CCursorRange::two(CCursor::new(0), CCursor::new(len))));
            state.store(ctx, self.editor_id);
        }
    }

    fn action_copy(&self, ctx: &egui::Context) {
        if let Some(state) = egui::TextEdit::load_state(ctx, self.editor_id) {
            if let Some(range) = state.cursor.char_range() {
                let start = range.primary.index.min(range.secondary.index);
                let end = range.primary.index.max(range.secondary.index);
                if end > start {
                    let text = char_range_substring(&self.code_text, start, end);
                    ctx.copy_text(text);
                }
            }
        }
    }

    fn action_cut(&mut self, ctx: &egui::Context) {
        if let Some(mut state) = egui::TextEdit::load_state(ctx, self.editor_id) {
            if let Some(range) = state.cursor.char_range() {
                let start = range.primary.index.min(range.secondary.index);
                let end = range.primary.index.max(range.secondary.index);
                if end > start {
                    let text = char_range_substring(&self.code_text, start, end);
                    ctx.copy_text(text);
                    self.code_text = delete_char_range(&self.code_text, start, end);
                }
                state
                    .cursor
                    .set_char_range(Some(egui::text::CCursorRange::one(egui::text::CCursor::new(
                        start,
                    ))));
                state.store(ctx, self.editor_id);
            }
        }
    }

    fn action_paste(&mut self, ctx: &egui::Context) {
        let clipboard = arboard::Clipboard::new()
            .ok()
            .and_then(|mut c| c.get_text().ok())
            .unwrap_or_default();
        if clipboard.is_empty() {
            return;
        }
        if let Some(mut state) = egui::TextEdit::load_state(ctx, self.editor_id) {
            if let Some(range) = state.cursor.char_range() {
                let start = range.primary.index.min(range.secondary.index);
                let end = range.primary.index.max(range.secondary.index);
                if end > start {
                    self.code_text = delete_char_range(&self.code_text, start, end);
                }
                self.code_text = insert_at_char_index(&self.code_text, start, &clipboard);
                let new_pos = start + clipboard.chars().count();
                state
                    .cursor
                    .set_char_range(Some(egui::text::CCursorRange::one(egui::text::CCursor::new(
                        new_pos,
                    ))));
                state.store(ctx, self.editor_id);
            }
        }
    }

    fn action_undo(&mut self, ctx: &egui::Context) {
        if let Some(mut state) = egui::TextEdit::load_state(ctx, self.editor_id) {
            let current = (
                state
                    .cursor
                    .char_range()
                    .unwrap_or(egui::text::CCursorRange::one(egui::text::CCursor::new(0))),
                self.code_text.clone(),
            );
            let mut undoer = state.undoer();
            if let Some((cursor_range, text)) = undoer.undo(&current).cloned() {
                self.code_text = text;
                state.cursor.set_char_range(Some(cursor_range));
                state.set_undoer(undoer);
                state.store(ctx, self.editor_id);
            }
        }
    }

    fn action_redo(&mut self, ctx: &egui::Context) {
        if let Some(mut state) = egui::TextEdit::load_state(ctx, self.editor_id) {
            let current = (
                state
                    .cursor
                    .char_range()
                    .unwrap_or(egui::text::CCursorRange::one(egui::text::CCursor::new(0))),
                self.code_text.clone(),
            );
            let mut undoer = state.undoer();
            if let Some((cursor_range, text)) = undoer.redo(&current).cloned() {
                self.code_text = text;
                state.cursor.set_char_range(Some(cursor_range));
                state.set_undoer(undoer);
                state.store(ctx, self.editor_id);
            }
        }
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

fn char_range_substring(text: &str, start_char: usize, end_char: usize) -> String {
    text.chars().skip(start_char).take(end_char - start_char).collect()
}

fn delete_char_range(text: &str, start_char: usize, end_char: usize) -> String {
    text.chars()
        .enumerate()
        .filter(|(i, _)| *i < start_char || *i >= end_char)
        .map(|(_, c)| c)
        .collect()
}

fn insert_at_char_index(text: &str, char_index: usize, insert: &str) -> String {
    let before: String = text.chars().take(char_index).collect();
    let after: String = text.chars().skip(char_index).collect();
    format!("{}{}{}", before, insert, after)
}
