use std::sync::Arc;

use eframe::egui::{
    self, epaint::StrokeKind, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, Vec2,
};
use jereide_core::{AppState, TAB_STRIP_HEIGHT};

const STRIP_BG: Color32 = Color32::from_rgb(215, 215, 215);
const ACTIVE_TAB_BG: Color32 = Color32::from_rgb(255, 255, 255);
const INACTIVE_TAB_BG: Color32 = Color32::from_rgb(238, 238, 238);
const ACTIVE_TEXT: Color32 = Color32::from_rgb(30, 30, 30);
const INACTIVE_TEXT: Color32 = Color32::from_rgb(130, 130, 130);
const BORDER: Color32 = Color32::from_rgb(200, 200, 200);
const CLOSE_BG_HOVER: Color32 = Color32::from_rgb(196, 196, 196);
const CLOSE_ICON: Color32 = Color32::from_rgb(120, 120, 120);
const CLOSE_ICON_HOVER: Color32 = Color32::from_rgb(60, 60, 60);
const MODIFIED_DOT: Color32 = egui::Color32::from_rgb(210, 150, 30);

const TAB_TOP_MARGIN: f32 = 3.0;
const TAB_PAD_LEFT: f32 = 2.0;
const TAB_PAD_RIGHT: f32 = 2.0;
const TAB_CORNER_RADIUS: u8 = 4;
const CLOSE_BTN_SIZE: f32 = 14.0;
const CLOSE_BTN_SPACING: f32 = 4.0;
const CLOSE_ICON_HALF: f32 = 3.0;
const CLOSE_STROKE: f32 = 1.2;
const MODIFIED_DOT_RADIUS: f32 = 3.0;
const MODIFIED_DOT_GAP: f32 = 4.0;

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
        let dot_extra = if has_dot { MODIFIED_DOT_RADIUS * 2.0 + MODIFIED_DOT_GAP } else { 0.0 };
        let left_req = TAB_PAD_LEFT + dot_extra;
        let right_req = CLOSE_BTN_SPACING + CLOSE_BTN_SIZE + TAB_PAD_RIGHT;
        let side = left_req.max(right_req);
        let tab_w = side + text_w + side;

        let tab_rect = Rect::from_min_size(Pos2::new(cursor_x, tab_top), Vec2::new(tab_w, tab_height));
        let text_pos = Pos2::new(tab_rect.center().x - text_w / 2.0, tab_rect.center().y - text_h / 2.0);
        let dot_pos = Pos2::new(text_pos.x - MODIFIED_DOT_GAP - MODIFIED_DOT_RADIUS, tab_rect.center().y);
        let close_rect = Rect::from_center_size(
            Pos2::new(text_pos.x + text_w + CLOSE_BTN_SPACING + CLOSE_BTN_SIZE / 2.0, tab_rect.center().y),
            Vec2::splat(CLOSE_BTN_SIZE),
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
    painter.rect_filled(strip_rect, 0, STRIP_BG);
    painter.hline(strip_rect.x_range(), tab_bottom, Stroke::new(1.0, BORDER));

    let rounding = CornerRadius { nw: TAB_CORNER_RADIUS, ne: TAB_CORNER_RADIUS, sw: 0, se: 0 };

    for idx in 0..state.tabs.len() {
        let layout = &layouts[idx];
        let is_active = idx == state.active_tab_index;

        let bg = if is_active { ACTIVE_TAB_BG } else { INACTIVE_TAB_BG };
        painter.rect(layout.rect, rounding, bg, Stroke::new(1.0, BORDER), StrokeKind::Inside);

        if is_active {
            painter.rect_filled(
                Rect::from_min_max(
                    Pos2::new(layout.rect.left(), tab_bottom - 1.0),
                    Pos2::new(layout.rect.right(), tab_bottom),
                ),
                0,
                ACTIVE_TAB_BG,
            );
        }

        let text_color = if is_active { ACTIVE_TEXT } else { INACTIVE_TEXT };
        painter.galley_with_override_text_color(layout.text_pos, layout.galley.clone(), text_color);

        if layout.has_dot {
            painter.circle_filled(layout.dot_pos, MODIFIED_DOT_RADIUS, MODIFIED_DOT);
        }

        if tab_hovered[idx] {
            if close_hovered[idx] {
                painter.rect_filled(layout.close_rect, 2, CLOSE_BG_HOVER);
            }
            let icon_color = if close_hovered[idx] { CLOSE_ICON_HOVER } else { CLOSE_ICON };
            let cx = layout.close_rect.center().x;
            let cy = layout.close_rect.center().y;
            painter.line_segment(
                [Pos2::new(cx - CLOSE_ICON_HALF, cy - CLOSE_ICON_HALF),
                 Pos2::new(cx + CLOSE_ICON_HALF, cy + CLOSE_ICON_HALF)],
                Stroke::new(CLOSE_STROKE, icon_color),
            );
            painter.line_segment(
                [Pos2::new(cx + CLOSE_ICON_HALF, cy - CLOSE_ICON_HALF),
                 Pos2::new(cx - CLOSE_ICON_HALF, cy + CLOSE_ICON_HALF)],
                Stroke::new(CLOSE_STROKE, icon_color),
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
