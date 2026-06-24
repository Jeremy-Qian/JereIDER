use std::sync::Arc;

use eframe::egui::{
    self, epaint::StrokeKind, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, Vec2,
};
use jereide_core::{
    AppState, TAB_ACTIVE_BG, TAB_ACTIVE_TEXT, TAB_BORDER, TAB_CLOSE_BG_HOVER, TAB_CLOSE_BTN_SIZE,
    TAB_CLOSE_BTN_SPACING, TAB_CLOSE_ICON, TAB_CLOSE_ICON_HALF, TAB_CLOSE_ICON_HOVER,
    TAB_CLOSE_STROKE, TAB_CORNER_RADIUS, TAB_INACTIVE_BG, TAB_INACTIVE_TEXT, TAB_MODIFIED_DOT,
    TAB_MODIFIED_DOT_GAP, TAB_MODIFIED_DOT_RADIUS, TAB_PAD_LEFT, TAB_PAD_RIGHT, TAB_STRIP_BG,
    TAB_STRIP_HEIGHT, TAB_TOP_MARGIN,
};

/// Pre-computed data for each tab so we don't re-measure or re-compute
/// during the paint phase.
struct TabLayout {
    rect: Rect,
    close_rect: Rect,
    text_pos: Pos2,
    has_dot: bool,
    dot_pos: Pos2,
    galley: Arc<egui::Galley>,
}

pub fn render_tab_strip(state: &mut AppState, ui: &mut egui::Ui) {
    let available = ui.available_size();
    let (strip_rect, strip_resp) = ui.allocate_exact_size(
        Vec2::new(available.x, TAB_STRIP_HEIGHT),
        Sense::click(),
    );
    let tab_bottom = strip_rect.bottom();
    let tab_top = strip_rect.top() + TAB_TOP_MARGIN;
    let tab_height = tab_bottom - tab_top;

    let font_id = FontId::proportional(12.0);

    let mut layouts: Vec<TabLayout> = Vec::with_capacity(state.tabs.len());
    let mut cursor_x = strip_rect.left();

    for idx in 0..state.tabs.len() {
        let tab = &state.tabs[idx];
        let name = tab.file_name();
        let galley = ui.fonts_mut(|f| f.layout_job(egui::text::LayoutJob::simple(name.clone(), font_id.clone(), Color32::WHITE, f32::INFINITY)));
        let text_w = galley.size().x;
        let text_h = galley.size().y;

        let has_dot = tab.is_modified();
        let dot_extra = if has_dot { TAB_MODIFIED_DOT_RADIUS * 2.0 + TAB_MODIFIED_DOT_GAP } else { 0.0 };
        let left_req = TAB_PAD_LEFT + dot_extra;
        let right_req = TAB_CLOSE_BTN_SPACING + TAB_CLOSE_BTN_SIZE + TAB_PAD_RIGHT;
        let side = left_req.max(right_req);
        let tab_w = side + text_w + side;

        let tab_rect = Rect::from_min_size(Pos2::new(cursor_x, tab_top), Vec2::new(tab_w, tab_height));
        let text_pos = Pos2::new(tab_rect.center().x - text_w / 2.0, tab_rect.center().y - text_h / 2.0);
        let dot_pos = Pos2::new(text_pos.x - TAB_MODIFIED_DOT_GAP - TAB_MODIFIED_DOT_RADIUS, tab_rect.center().y);
        let close_rect = Rect::from_center_size(
            Pos2::new(text_pos.x + text_w + TAB_CLOSE_BTN_SPACING + TAB_CLOSE_BTN_SIZE / 2.0, tab_rect.center().y),
            Vec2::splat(TAB_CLOSE_BTN_SIZE),
        );

        layouts.push(TabLayout { rect: tab_rect, close_rect, text_pos, has_dot, dot_pos, galley });
        cursor_x = tab_rect.right();
    }

    let mut click_tab: Option<usize> = None;
    let mut close_tab: Option<usize> = None;
    let mut close_hovered = vec![false; state.tabs.len()];
    let mut tab_hovered = vec![false; state.tabs.len()];

    for idx in 0..state.tabs.len() {
        let tab_id = egui::Id::new(("tab", idx));
        let close_id = egui::Id::new(("close", idx));

        let tab_resp = ui.interact(layouts[idx].rect, tab_id, Sense::click());
        let close_resp = ui.interact(layouts[idx].close_rect, close_id, Sense::click());

        close_hovered[idx] = close_resp.hovered();
        tab_hovered[idx] = tab_resp.hovered() || close_resp.hovered();

        if close_resp.clicked() {
            close_tab = Some(idx);
        } else if tab_resp.clicked() {
            click_tab = Some(idx);
        }
    }

    let painter = ui.painter();

    // Strip background and bottom separator
    painter.rect_filled(strip_rect, 0, TAB_STRIP_BG);
    painter.hline(strip_rect.x_range(), tab_bottom, Stroke::new(1.0, TAB_BORDER));

    let rounding = CornerRadius { nw: TAB_CORNER_RADIUS, ne: TAB_CORNER_RADIUS, sw: 0, se: 0 };

    for idx in 0..state.tabs.len() {
        let layout = &layouts[idx];
        let is_active = idx == state.active_tab_index;

        let bg = if is_active { TAB_ACTIVE_BG } else { TAB_INACTIVE_BG };
        painter.rect(layout.rect, rounding, bg, Stroke::new(1.0, TAB_BORDER), StrokeKind::Inside);

        if is_active {
            painter.rect_filled(
                Rect::from_min_max(
                    Pos2::new(layout.rect.left(), tab_bottom - 1.0),
                    Pos2::new(layout.rect.right(), tab_bottom),
                ),
                0,
                TAB_ACTIVE_BG,
            );
        }

        let text_color = if is_active { TAB_ACTIVE_TEXT } else { TAB_INACTIVE_TEXT };
        painter.galley_with_override_text_color(layout.text_pos, layout.galley.clone(), text_color);

        if layout.has_dot {
            painter.circle_filled(layout.dot_pos, TAB_MODIFIED_DOT_RADIUS, TAB_MODIFIED_DOT);
        }

        if tab_hovered[idx] {
            if close_hovered[idx] {
                painter.rect_filled(layout.close_rect, 2, TAB_CLOSE_BG_HOVER);
            }
            let icon_color = if close_hovered[idx] { TAB_CLOSE_ICON_HOVER } else { TAB_CLOSE_ICON };
            let cx = layout.close_rect.center().x;
            let cy = layout.close_rect.center().y;
            painter.line_segment(
                [Pos2::new(cx - TAB_CLOSE_ICON_HALF, cy - TAB_CLOSE_ICON_HALF),
                 Pos2::new(cx + TAB_CLOSE_ICON_HALF, cy + TAB_CLOSE_ICON_HALF)],
                Stroke::new(TAB_CLOSE_STROKE, icon_color),
            );
            painter.line_segment(
                [Pos2::new(cx + TAB_CLOSE_ICON_HALF, cy - TAB_CLOSE_ICON_HALF),
                 Pos2::new(cx - TAB_CLOSE_ICON_HALF, cy + TAB_CLOSE_ICON_HALF)],
                Stroke::new(TAB_CLOSE_STROKE, icon_color),
            );
        }
    }

    if strip_resp.double_clicked() {
        state.new_tab();
    }
    if let Some(idx) = close_tab {
        state.close_tab(idx);
    }
    if let Some(idx) = click_tab {
        state.active_tab_index = idx;
    }
}
