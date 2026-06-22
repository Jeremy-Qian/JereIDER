use eframe::egui;
use jereide_core::{char_range_substring, delete_char_range, insert_at_char_index, AppState};

// All the actions are handled here
pub fn handle_edit_action(state: &mut AppState, ctx: &egui::Context, action: &str) {
    match action {
        "select_all" => action_select_all(state, ctx),
        "copy" => action_copy(state, ctx),
        "cut" => action_cut(state, ctx),
        "paste" => action_paste(state, ctx),
        "undo" => action_undo(state, ctx),
        "redo" => action_redo(state, ctx),
        "githubstar" => action_github_star(state, ctx),
        _ => {
            // This isn't supposed to happen, but who knows?
            eprintln!("Unknown edit action: '{}'", action);
        }
    }
}

/// Selects everything.
fn action_select_all(state: &AppState, ctx: &egui::Context) {
    if let Some(mut edit_state) = egui::TextEdit::load_state(ctx, state.editor_id) {
        let len = state.code_text.chars().count();
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
                let text = char_range_substring(&state.code_text, start, end);
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
                let text = char_range_substring(&state.code_text, start, end);
                ctx.copy_text(text);
                state.code_text = delete_char_range(&state.code_text, start, end);
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

/// Pastes from the clipboard.
fn action_paste(state: &mut AppState, ctx: &egui::Context) {
    let clipboard = arboard::Clipboard::new()
        .ok()
        .and_then(|mut c| c.get_text().ok())
        .unwrap_or_default();
    if clipboard.is_empty() {
        return;
    }
    if let Some(mut edit_state) = egui::TextEdit::load_state(ctx, state.editor_id) {
        if let Some(range) = edit_state.cursor.char_range() {
            let start = range.primary.index.min(range.secondary.index);
            let end = range.primary.index.max(range.secondary.index);
            if end > start {
                state.code_text = delete_char_range(&state.code_text, start, end);
            }
            state.code_text = insert_at_char_index(&state.code_text, start, &clipboard);
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
        let current = (
            edit_state
                .cursor
                .char_range()
                .unwrap_or(egui::text::CCursorRange::one(egui::text::CCursor::new(0))),
            state.code_text.clone(),
        );
        let mut undoer = edit_state.undoer();
        if let Some((cursor_range, text)) = undoer.undo(&current).cloned() {
            state.code_text = text;
            edit_state.cursor.set_char_range(Some(cursor_range));
            edit_state.set_undoer(undoer);
            edit_state.store(ctx, state.editor_id);
        }
    }
}
/// Redo
fn action_redo(state: &mut AppState, ctx: &egui::Context) {
    if let Some(mut edit_state) = egui::TextEdit::load_state(ctx, state.editor_id) {
        let current = (
            edit_state
                .cursor
                .char_range()
                .unwrap_or(egui::text::CCursorRange::one(egui::text::CCursor::new(0))),
            state.code_text.clone(),
        );
        let mut undoer = edit_state.undoer();
        if let Some((cursor_range, text)) = undoer.redo(&current).cloned() {
            state.code_text = text;
            edit_state.cursor.set_char_range(Some(cursor_range));
            edit_state.set_undoer(undoer);
            edit_state.store(ctx, state.editor_id);
        }
    }
}
/// Star us on GitHub!
fn action_github_star(_state: &AppState, ctx: &egui::Context) {
    ctx.open_url(egui::OpenUrl {
        /// TODO: Use real URL instead
        ///
        url: String::from("https://github.com/jeremy-qian/jereide"),
        new_tab: true,
    });
}
