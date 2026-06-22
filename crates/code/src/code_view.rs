use std::cell::RefCell;
use std::sync::Arc;

use eframe::egui;
use jereide_core::{char_index_to_line_col, AppState};
use jereide_syntax::SyntaxHighlighter;

// Persisting these across frames is essential — the SyntaxHighlighter
// builds up a per-line incremental cache that would be wasted if we
// recreated it every frame.
thread_local! {
    static HIGHLIGHTER: RefCell<Option<SyntaxHighlighter>> = const { RefCell::new(None) };
    static PREV_EXTENSION: RefCell<Option<String>> = const { RefCell::new(None) };
    static CACHED_TEXT: RefCell<String> = const { RefCell::new(String::new()) };
    static CACHED_GALLEY: RefCell<Option<Arc<egui::Galley>>> = const { RefCell::new(None) };
}

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

    // ── Extension detection (recreates highlighter when the file type changes) ──
    let extension = state
        .current_file_path
        .as_ref()
        .and_then(|p| std::path::Path::new(p).extension())
        .and_then(|ext| ext.to_str());

    let ext_changed = PREV_EXTENSION.with(|prev| {
        let mut prev = prev.borrow_mut();
        if prev.as_deref() != extension {
            *prev = extension.map(String::from);
            true
        } else {
            false
        }
    });

    if ext_changed {
        HIGHLIGHTER.with(|hl| {
            *hl.borrow_mut() = Some(SyntaxHighlighter::new(14.0, extension));
        });
        CACHED_TEXT.with(|t| t.borrow_mut().clear());
        CACHED_GALLEY.with(|g| *g.borrow_mut() = None);
    }

    // ── Initialise highlighter on first frame ──
    HIGHLIGHTER.with(|hl| {
        if hl.borrow().is_none() {
            *hl.borrow_mut() = Some(SyntaxHighlighter::new(14.0, extension));
        }
    });

    // ── Layouter with Galley-level caching ──
    // On cache hit we return the same Arc<Galley>, letting egui skip
    // text shaping entirely.  Only when the text actually changes do we
    // call highlight() and fonts_mut().layout_job().
    let mut layouter =
        |ui: &egui::Ui, text: &dyn egui::widgets::TextBuffer, _max_width: f32| {
            let text_str = text.as_str();

            // Fast path: text unchanged → return cached Galley
            if CACHED_TEXT.with(|t| t.borrow().as_str() == text_str) {
                if let Some(galley) = CACHED_GALLEY.with(|g| g.borrow().clone()) {
                    return galley;
                }
            }

            // Slow path: text changed — highlight and shape
            let layout_job = HIGHLIGHTER.with(|hl| {
                hl.borrow_mut()
                    .as_mut()
                    .expect("highlighter initialized")
                    .highlight(text_str)
            });
            let galley = ui.fonts_mut(|f| f.layout_job(layout_job));

            CACHED_TEXT.with(|t| *t.borrow_mut() = text_str.to_string());
            CACHED_GALLEY.with(|g| *g.borrow_mut() = Some(galley.clone()));

            galley
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
