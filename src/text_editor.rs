use eframe::egui;
use crate::JereIDEApp;

impl JereIDEApp {
    pub fn handle_edit_action(&mut self, ctx: &egui::Context, action: &str) {
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
