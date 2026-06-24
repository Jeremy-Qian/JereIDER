use std::cell::RefCell;
use std::sync::Arc;

use eframe::egui;
use jereide_core::{
    char_index_to_line_col, AppState, MAIN_CORNER_RADIUS, CURRENT_LINE_BG, EDITOR_BG,
    EDITOR_FONT_SIZE, EDITOR_INNER_MARGIN_BOTTOM, EDITOR_INNER_MARGIN_LEFT_EXTRA,
    EDITOR_INNER_MARGIN_RIGHT, EDITOR_INNER_MARGIN_TOP, GUTTER_BG,
    GUTTER_DIGIT_WIDTH, GUTTER_HIGHLIGHT_OFFSET,
    GUTTER_LINE_NUMBER_RIGHT_OFFSET, GUTTER_PADDING_LEFT, GUTTER_PADDING_RIGHT,
    GUTTER_TEXT, GUTTER_TEXT_CURRENT, SCROLL_BAR_WIDTH,
};
use jereide_syntax::SyntaxHighlighter;

// A new thread for the syntax highlighting, I guess.
thread_local! {
    static HIGHLIGHTER: RefCell<Option<SyntaxHighlighter>> = const { RefCell::new(None) };
    static PREV_EXTENSION: RefCell<Option<String>> = const { RefCell::new(None) };

    static CUR_GALLEY: RefCell<Option<Arc<egui::Galley>>> = const { RefCell::new(None) };
}

// Pretty useless functions, but...
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
        let mut s = egui::style::ScrollStyle::solid();
        s.bar_width = SCROLL_BAR_WIDTH;
        s
    };

    // Capture the active tab index once for direct field access (avoids
    // borrow conflicts that helper methods would cause with the TextEdit).
    let active_idx = state.active_tab_index;

    // Incremental Highlighting to make JereIDE faster
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
    let row_height = ui.fonts_mut(|f| f.row_height(&font_id));
    let line_count = visual_line_count(&state.tabs[active_idx].text);
    let gutter_w = gutter_width(line_count);
    let cursor_line = state.tabs[active_idx].cursor_line;

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

            CUR_GALLEY.with(|g| {
                *g.borrow_mut() = Some(galley.clone());
            });

            galley
        };
    // Scrolling both ways
    // TODO: Add option for wrap instead of horizontal scroll
    let response = egui::ScrollArea::both()
        .auto_shrink(false)
        .show(ui, |ui| {
            let viewport = ui.max_rect().size();
            ui.set_min_size(viewport);

            let widget_top = ui.cursor().min.y;
            // Complicated painting
            if cursor_line > 0 && cursor_line <= line_count {
                let y = CUR_GALLEY.with(|g| {
                    let inner_margin_top = EDITOR_INNER_MARGIN_TOP as f32;
                    if let Some(galley) = g.borrow().as_ref() {
                        let idx = cursor_line.saturating_sub(1);
                        if idx < galley.rows.len() {
                            widget_top + inner_margin_top + galley.rows[idx].pos.y
                        } else {
                            widget_top + inner_margin_top + idx as f32 * row_height
                        }
                    } else {
                        let idx = cursor_line.saturating_sub(1);
                        widget_top + inner_margin_top + idx as f32 * row_height
                    }
                });
                let hl_x = gutter_w + GUTTER_HIGHLIGHT_OFFSET;
                let hl_w = (viewport.x - gutter_w - GUTTER_HIGHLIGHT_OFFSET).max(0.0);
                let painter = ui.painter();
                // Current Line Highlighting
                painter.rect_filled(
                    egui::Rect::from_min_size(egui::pos2(hl_x, y), egui::vec2(hl_w, row_height)),
                    MAIN_CORNER_RADIUS,
                    CURRENT_LINE_BG,
                );
            }
            // The Code Editor(TextEdit::code_editor captures Tabs and keeps focus)
            let text_response = ui.add(
                egui::TextEdit::code_editor(
                    egui::TextEdit::multiline(&mut state.tabs[active_idx].text),
                )
                .id_source("editor")
                .desired_width(viewport.x)
                .frame(egui::Frame {
                    inner_margin: egui::Margin {
                        left: (gutter_w + EDITOR_INNER_MARGIN_LEFT_EXTRA as f32) as i8,
                        right: EDITOR_INNER_MARGIN_RIGHT,
                        top: EDITOR_INNER_MARGIN_TOP,
                        bottom: EDITOR_INNER_MARGIN_BOTTOM,
                    },
                    ..egui::Frame::NONE
                })
                .layouter(&mut layouter),
            );

            let text_alloc = text_response.rect;
            let gutter_y0 = text_alloc.top();
            let gutter_y1 = text_alloc.bottom().max(ui.clip_rect().bottom());

            {
                let painter = ui.painter();

                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(0.0, gutter_y0),
                        egui::vec2(gutter_w, gutter_y1 - gutter_y0),
                    ),
                    MAIN_CORNER_RADIUS,
                    GUTTER_BG,
                );

                CUR_GALLEY.with(|g| {
                    if let Some(galley) = g.borrow().as_ref() {
                        for (i, row) in galley.rows.iter().enumerate() {
                            let line_num = i + 1;
                            let y = widget_top + EDITOR_INNER_MARGIN_TOP as f32 + row.pos.y;
                            let is_current = line_num == cursor_line;
                            let color = if is_current {
                                GUTTER_TEXT_CURRENT
                            } else {
                                GUTTER_TEXT
                            };
                            painter.text(
                                egui::pos2(gutter_w - GUTTER_LINE_NUMBER_RIGHT_OFFSET, y),
                                egui::Align2::RIGHT_TOP,
                                line_num.to_string(),
                                font_id.clone(),
                                color,
                            );
                        }
                    }
                });
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
