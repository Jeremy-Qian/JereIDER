use eframe::egui;
use jereide_core::{char_range_substring, delete_char_range, insert_at_char_index, AppState};

/// Type-safe edit actions that the menu system can dispatch to the editor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditAction {
    SelectAll,
    Copy,
    Cut,
    Paste,
    Undo,
    Redo,
}

impl EditAction {
    /// Convert a menu event string to an edit action, if it matches.
    pub fn from_menu_id(id: &str) -> Option<Self> {
        match id {
            "select_all" => Some(Self::SelectAll),
            "copy" => Some(Self::Copy),
            "cut" => Some(Self::Cut),
            "paste" => Some(Self::Paste),
            "undo" => Some(Self::Undo),
            "redo" => Some(Self::Redo),
            _ => None,
        }
    }
}

/// Dispatch an edit action.
pub fn handle_edit_action(state: &mut AppState, ctx: &egui::Context, action: EditAction) {
    match action {
        EditAction::SelectAll => action_select_all(state, ctx),
        EditAction::Copy => action_copy(state, ctx),
        EditAction::Cut => action_cut(state, ctx),
        EditAction::Paste => action_paste(state, ctx),
        EditAction::Undo => action_undo(state, ctx),
        EditAction::Redo => action_redo(state, ctx),
    }
}

/// Selects everything.
fn action_select_all(state: &AppState, ctx: &egui::Context) {
    if let Some(mut edit_state) = egui::TextEdit::load_state(ctx, state.editor_id) {
        let len = state.current_tab().text.chars().count();
        use egui::text::{CCursor, CCursorRange};
        edit_state
            .cursor
            .set_char_range(Some(CCursorRange::two(CCursor::new(0), CCursor::new(len))));
        edit_state.store(ctx, state.editor_id);
    }
}

/// Copies selected.
fn action_copy(state: &AppState, ctx: &egui::Context) {
    if let Some(edit_state) = egui::TextEdit::load_state(ctx, state.editor_id) {
        if let Some(range) = edit_state.cursor.char_range() {
            let start = range.primary.index.min(range.secondary.index);
            let end = range.primary.index.max(range.secondary.index);
            if end > start {
                let text = char_range_substring(state.current_tab().text.as_str(), start, end);
                ctx.copy_text(text);
            }
        }
    }
}

/// Cuts selected.
fn action_cut(state: &mut AppState, ctx: &egui::Context) {
    if let Some(mut edit_state) = egui::TextEdit::load_state(ctx, state.editor_id) {
        if let Some(range) = edit_state.cursor.char_range() {
            let start = range.primary.index.min(range.secondary.index);
            let end = range.primary.index.max(range.secondary.index);
            if end > start {
                let idx = state.active_tab_index;
                let text = char_range_substring(&state.tabs[idx].text, start, end);
                ctx.copy_text(text);
                let new_text = delete_char_range(&state.tabs[idx].text, start, end);
                state.tabs[idx].text = new_text;
            }
            edit_state
                .cursor
                .set_char_range(Some(egui::text::CCursorRange::one(
                    egui::text::CCursor::new(start),
                )));
            edit_state.store(ctx, state.editor_id);
        }
    }
}

/// Returns the current clipboard text, reusing a single cached clipboard instance.
fn clipboard_text() -> String {
    use std::sync::Mutex;
    static CLIPBOARD: std::sync::OnceLock<Mutex<arboard::Clipboard>> =
        std::sync::OnceLock::new();
    CLIPBOARD
        .get_or_init(|| {
            Mutex::new(
                arboard::Clipboard::new()
                    .expect("failed to initialize system clipboard"),
            )
        })
        .lock()
        .ok()
        .and_then(|mut cb| cb.get_text().ok())
        .unwrap_or_default()
}

/// Pastes from the clipboard.
fn action_paste(state: &mut AppState, ctx: &egui::Context) {
    let clipboard = clipboard_text();
    if clipboard.is_empty() {
        return;
    }
    if let Some(mut edit_state) = egui::TextEdit::load_state(ctx, state.editor_id) {
        if let Some(range) = edit_state.cursor.char_range() {
            let start = range.primary.index.min(range.secondary.index);
            let end = range.primary.index.max(range.secondary.index);
            let idx = state.active_tab_index;
            let text = if end > start {
                let deleted = delete_char_range(&state.tabs[idx].text, start, end);
                insert_at_char_index(&deleted, start, &clipboard)
            } else {
                insert_at_char_index(&state.tabs[idx].text, start, &clipboard)
            };
            state.tabs[idx].text = text;
            let new_pos = start + clipboard.chars().count();
            edit_state
                .cursor
                .set_char_range(Some(egui::text::CCursorRange::one(
                    egui::text::CCursor::new(new_pos),
                )));
            edit_state.store(ctx, state.editor_id);
        }
    }
}

/// Undoes last action(this is pretty complicated)
fn action_undo(state: &mut AppState, ctx: &egui::Context) {
    if let Some(mut edit_state) = egui::TextEdit::load_state(ctx, state.editor_id) {
        let idx = state.active_tab_index;
        let current = (
            edit_state
                .cursor
                .char_range()
                .unwrap_or(egui::text::CCursorRange::one(egui::text::CCursor::new(0))),
            state.tabs[idx].text.clone(),
        );
        let mut undoer = edit_state.undoer();
        if let Some((cursor_range, text)) = undoer.undo(&current).cloned() {
            state.tabs[idx].text = text;
            edit_state.cursor.set_char_range(Some(cursor_range));
            edit_state.set_undoer(undoer);
            edit_state.store(ctx, state.editor_id);
        }
    }
}
/// Redo
fn action_redo(state: &mut AppState, ctx: &egui::Context) {
    if let Some(mut edit_state) = egui::TextEdit::load_state(ctx, state.editor_id) {
        let idx = state.active_tab_index;
        let current = (
            edit_state
                .cursor
                .char_range()
                .unwrap_or(egui::text::CCursorRange::one(egui::text::CCursor::new(0))),
            state.tabs[idx].text.clone(),
        );
        let mut undoer = edit_state.undoer();
        if let Some((cursor_range, text)) = undoer.redo(&current).cloned() {
            state.tabs[idx].text = text;
            edit_state.cursor.set_char_range(Some(cursor_range));
            edit_state.set_undoer(undoer);
            edit_state.store(ctx, state.editor_id);
        }
    }
}

