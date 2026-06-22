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

    /// Galley from the latest layouter call — holds exact row positions for
    /// the gutter to read after the TextEdit renders.
    static CUR_GALLEY: RefCell<Option<Arc<egui::Galley>>> = const { RefCell::new(None) };
}

/// Approximate pixel width per digit in the monospace font at 14px.
const DIGIT_W: f32 = 8.0;

/// Returns the number of visual lines in the text (counting trailing newline).
fn visual_line_count(text: &str) -> usize {
    if text.is_empty() {
        1
    } else {
        text.chars().filter(|&c| c == '\n').count() + 1
    }
}

/// Returns the gutter width in pixels needed for the given line count.
fn gutter_width(line_count: usize) -> f32 {
    let digit_count = (line_count as f64).log10().floor() as usize + 1;
    10.0 + digit_count as f32 * DIGIT_W + 6.0
}

/// Renders the code editor view — a syntax-highlighted multi-line text editor
/// with a line-number gutter and cursor-line highlighting.
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
    }

    // ── Initialise highlighter on first frame ──
    HIGHLIGHTER.with(|hl| {
        if hl.borrow().is_none() {
            *hl.borrow_mut() = Some(SyntaxHighlighter::new(14.0, extension));
        }
    });

    // ── Font & layout metrics (computed once per frame) ──
    let font_id = egui::FontId::monospace(14.0);
    let row_height = ui.fonts_mut(|f| f.row_height(&font_id));
    let line_count = visual_line_count(&state.code_text);
    let gutter_w = gutter_width(line_count);

    // ── Layouter ──
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

            // Store galley for the gutter.
            CUR_GALLEY.with(|g| {
                *g.borrow_mut() = Some(galley.clone());
            });

            galley
        };

    let response = egui::ScrollArea::both()
        .auto_shrink(false)
        .show(ui, |ui| {
            let viewport = ui.max_rect().size();
            ui.set_min_size(viewport);

            // Capture the widget allocation position BEFORE adding the TextEdit
            // so we can paint the highlight *behind* the text.
            let widget_top = ui.cursor().min.y;

            // ── Current line highlight (BEHIND text) ──
            // Uses the PREVIOUS frame's galley row positions for pixel-accurate
            // alignment rather than computed row_height (which may differ from
            // the galley's actual row height by fractions of a pixel).
            if state.cursor_line > 0 && state.cursor_line <= line_count {
                let y = CUR_GALLEY.with(|g| {
                    let inner_margin_top = 10.0;
                    if let Some(galley) = g.borrow().as_ref() {
                        let idx = state.cursor_line.saturating_sub(1);
                        if idx < galley.rows.len() {
                            widget_top + inner_margin_top + galley.rows[idx].pos.y
                        } else {
                            widget_top + inner_margin_top + idx as f32 * row_height
                        }
                    } else {
                        let idx = state.cursor_line.saturating_sub(1);
                        widget_top + inner_margin_top + idx as f32 * row_height
                    }
                });
                let hl_x = gutter_w + 2.0;
                let hl_w = (viewport.x - gutter_w - 2.0).max(0.0);
                let painter = ui.painter();
                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(hl_x, y),
                        egui::vec2(hl_w, row_height),
                    ),
                    0.0,
                    egui::Color32::from_rgb(255, 255, 208),
                );
            }

            // ── TextEdit ──
            let text_response = ui.add(
                egui::TextEdit::code_editor(egui::TextEdit::multiline(&mut state.code_text))
                    .id_source("editor")
                    .desired_width(viewport.x)
                    .frame(egui::Frame {
                        inner_margin: egui::Margin {
                            left: (gutter_w + 6.0) as i8,
                            right: 10,
                            top: 10,
                            bottom: 10,
                        },
                        ..egui::Frame::NONE
                    })
                    .layouter(&mut layouter),
            );

            // ── Gutter & separator (painted AFTER TextEdit, ON TOP) ──
            let text_alloc = text_response.rect;
            // Gutter fills from the editor's top to the bottom of the visible
            // viewport — this covers empty space below short text while staying
            // within the editor area (no title-bar/status-bar leakage).
            let gutter_y0 = text_alloc.top();
            let gutter_y1 = text_alloc.bottom().max(ui.clip_rect().bottom());

            {
                let painter = ui.painter();

                // Gutter background
                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(0.0, gutter_y0),
                        egui::vec2(gutter_w, gutter_y1 - gutter_y0),
                    ),
                    0.0,
                    egui::Color32::from_rgb(245, 245, 245),
                );

                // Separator line
                painter.vline(
                    gutter_w,
                    gutter_y0..=gutter_y1,
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(224, 224, 224)),
                );

                // Line numbers
                CUR_GALLEY.with(|g| {
                    if let Some(galley) = g.borrow().as_ref() {
                        for (i, row) in galley.rows.iter().enumerate() {
                            let line_num = i + 1;
                            // Text origin inside the editor: widget_top + inner_margin.top
                            let y = widget_top + 10.0 + row.pos.y;
                            let is_current = line_num == state.cursor_line;
                            let color = if is_current {
                                egui::Color32::from_rgb(48, 48, 48)
                            } else {
                                egui::Color32::from_rgb(145, 145, 145)
                            };
                            painter.text(
                                egui::pos2(gutter_w - 5.0, y),
                                egui::Align2::RIGHT_TOP,
                                line_num.to_string(),
                                font_id.clone(),
                                color,
                            );
                        }
                    }
                });
            }

            // If there's empty space below the text, make it clickable.
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

    // ── Update cursor from TextEdit state ──
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


