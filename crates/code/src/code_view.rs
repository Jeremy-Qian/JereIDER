use std::cell::RefCell;
use std::sync::Arc;

use eframe::egui;
use jereide_core::{
    char_index_to_line_col, AppState, MAIN_CORNER_RADIUS, EDITOR_BG,
    EDITOR_FONT_SIZE, EDITOR_INNER_MARGIN_BOTTOM, EDITOR_INNER_MARGIN_LEFT_EXTRA,
    EDITOR_INNER_MARGIN_RIGHT, EDITOR_INNER_MARGIN_TOP, GUTTER_BG,
    GUTTER_DIGIT_WIDTH,
    GUTTER_LINE_NUMBER_RIGHT_OFFSET, GUTTER_PADDING_LEFT, GUTTER_PADDING_RIGHT,
    GUTTER_TEXT, GUTTER_TEXT_CURRENT, SCROLL_BAR_WIDTH,
};
use jereide_syntax::SyntaxHighlighter;

thread_local! {
    static HIGHLIGHTER: RefCell<Option<SyntaxHighlighter>> = const { RefCell::new(None) };
    static PREV_EXTENSION: RefCell<Option<String>> = const { RefCell::new(None) };
}

fn visual_line_count(text: &str) -> usize {
    if text.is_empty() {
        1
    } else {
        text.chars().filter(|&c| c == '\n').count() + 1
    }
}

fn gutter_width(line_count: usize) -> f32 {
    let digit_count = (line_count as f64).log10().floor() as usize + 1;
    GUTTER_PADDING_LEFT + digit_count as f32 * GUTTER_DIGIT_WIDTH + GUTTER_PADDING_RIGHT
}

pub fn render_code_view(state: &mut AppState, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();

    let style = ui.style_mut();
    style.visuals.extreme_bg_color = EDITOR_BG;
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
    style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
    style.spacing.scroll = {
        let mut s = egui::style::ScrollStyle::floating();
        s.bar_width = SCROLL_BAR_WIDTH;
        s
    };

    let active_idx = state.active_tab_index;

    let extension = state.tabs[active_idx]
        .file_path
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
            *hl.borrow_mut() = Some(SyntaxHighlighter::new(EDITOR_FONT_SIZE, extension));
        });
    }

    HIGHLIGHTER.with(|hl| {
        if hl.borrow().is_none() {
            *hl.borrow_mut() = Some(SyntaxHighlighter::new(EDITOR_FONT_SIZE, extension));
        }
    });

    let font_id = egui::FontId::monospace(EDITOR_FONT_SIZE);
    let line_count = visual_line_count(&state.tabs[active_idx].text);
    let gutter_w = gutter_width(line_count);
    let cursor_line = state.tabs[active_idx].cursor_line;

    let last_galley: RefCell<Option<Arc<egui::Galley>>> = RefCell::new(None);

    let mut layouter =
        |layouter_ui: &egui::Ui, text: &dyn egui::widgets::TextBuffer, wrap_width: f32| {
            let text_str = text.as_str();

            let mut layout_job = HIGHLIGHTER.with(|hl| {
                hl.borrow_mut()
                    .as_mut()
                    .expect("highlighter initialized")
                    .highlight(text_str)
            });

            layout_job.wrap.max_width = wrap_width;
            let galley = layouter_ui.fonts_mut(|f| f.layout_job(layout_job));
            *last_galley.borrow_mut() = Some(galley.clone());
            galley
        };

    let response = egui::ScrollArea::both()
        .auto_shrink(false)
        .show(ui, |ui| {
            let viewport = ui.max_rect().size();
            ui.set_min_size(viewport);

            let widget_top = ui.cursor().min.y;

            let horiz = ui.horizontal_top(|ui| {
                let (gutter_rect, _) = ui.allocate_exact_size(
                    egui::vec2(gutter_w, viewport.y),
                    egui::Sense::hover(),
                );

                let text_response = ui.add(
                    egui::TextEdit::code_editor(
                        egui::TextEdit::multiline(&mut state.tabs[active_idx].text),
                    )
                    .id_source("editor")
                    .desired_width(viewport.x - gutter_w)
                    .frame(egui::Frame {
                        inner_margin: egui::Margin {
                            left: EDITOR_INNER_MARGIN_LEFT_EXTRA,
                            right: EDITOR_INNER_MARGIN_RIGHT,
                            top: EDITOR_INNER_MARGIN_TOP,
                            bottom: EDITOR_INNER_MARGIN_BOTTOM,
                        },
                        ..egui::Frame::NONE
                    })
                    .layouter(&mut layouter),
                );

                (gutter_rect, text_response)
            });

            let (gutter_rect, text_response) = horiz.inner;
            let text_alloc = text_response.rect;

            let g_bottom = text_alloc.bottom().max(ui.clip_rect().bottom());
            let painter = ui.painter();
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(gutter_rect.left(), gutter_rect.top()),
                    egui::vec2(gutter_w, g_bottom - gutter_rect.top()),
                ),
                MAIN_CORNER_RADIUS,
                GUTTER_BG,
            );

            let line_start_y = widget_top + EDITOR_INNER_MARGIN_TOP as f32;
            if let Some(galley) = last_galley.borrow().as_ref() {
                for (i, row) in galley.rows.iter().enumerate() {
                    let line_y = line_start_y + row.pos.y;
                    let line_num = i + 1;
                    let is_current = line_num == cursor_line;
                    let color = if is_current {
                        GUTTER_TEXT_CURRENT
                    } else {
                        GUTTER_TEXT
                    };
                    painter.text(
                        egui::pos2(gutter_w - GUTTER_LINE_NUMBER_RIGHT_OFFSET, line_y),
                        egui::Align2::RIGHT_TOP,
                        line_num.to_string(),
                        font_id.clone(),
                        color,
                    );
                }
            }

            // Fill up the whole Y available space
            let remaining = ui.available_size();
            if remaining.y > 0.0 {
                let (_, bg) = ui.allocate_exact_size(remaining, egui::Sense::click());
                if bg.clicked() {
                    text_response.request_focus();
                }
                bg.on_hover_cursor(egui::CursorIcon::Text);
            }

            text_response
        })
        .inner;
    state.editor_id = response.id;
    // For the status bar Line/Col indicator
    if let Some(edit_state) = egui::TextEdit::load_state(&ctx, response.id) {
        if let Some(range) = edit_state.cursor.char_range() {
            let (line, col) =
                char_index_to_line_col(&state.tabs[active_idx].text, range.primary.index);
            state.tabs[active_idx].cursor_line = line;
            state.tabs[active_idx].cursor_col = col;
        }
    }

    if !state.editor_focused {
        state.editor_focused = true;
        response.request_focus();
    }
}
