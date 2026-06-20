use eframe::egui;

/// Easing function: OutCubic — matches Qt's QEasingCurve::OutCubic
fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powf(3.0)
}

/// Manages the slide animation state for a multi-page panel.
pub struct SlideAnimation {
    /// Which page index the slide animation started from
    pub from_index: usize,
    /// Which page index the slide animation is heading to
    pub to_index: usize,
    /// Current animation progress (0.0 = at from, 1.0 = at to)
    pub t: f32,
    /// Whether an animation is currently in progress
    pub animating: bool,
}

impl SlideAnimation {
    pub fn new(initial_page: usize) -> Self {
        Self {
            from_index: initial_page,
            to_index: initial_page,
            t: 1.0,
            animating: false,
        }
    }

    /// Start sliding to `target_index`. If we're already animating, the
    /// current visual offset is captured as the new `from_index` (inferred
    /// from the lerp position).
    pub fn slide_to(&mut self, target_index: usize) {
        if target_index == self.to_index && !self.animating {
            return;
        }
        self.from_index = self.to_index;
        self.to_index = target_index;
        self.t = 0.0;
        self.animating = true;
    }

    /// Advance the animation by `dt` seconds. Call this every frame.
    pub fn advance(&mut self, dt: f32) {
        if !self.animating {
            return;
        }
        self.t = (self.t + dt / 0.3).min(1.0); // 300 ms duration
        if self.t >= 1.0 {
            self.animating = false;
            self.from_index = self.to_index;
        }
    }

    /// Returns the visual scroll offset in pixels for the given viewport width.
    pub fn offset(&self, viewport_width: f32) -> f32 {
        if self.t >= 1.0 && !self.animating {
            return self.to_index as f32 * viewport_width;
        }
        let progress = ease_out_cubic(self.t);
        let from = self.from_index as f32 * viewport_width;
        let to = self.to_index as f32 * viewport_width;
        egui::lerp(from..=to, progress)
    }

    /// The "dominant" page index (the one the user perceives as current).
    /// While animating we report the target; once done, `from == to`.
    pub fn current_index(&self) -> usize {
        self.to_index
    }
}

/// Renders a sliding panel with animated page transitions.
///
/// * `ui` — the parent egui Ui.
/// * `page_count` — total number of pages.
/// * `offset` — the horizontal scroll offset (computed from `SlideAnimation`).
/// * `render_page` — called for each visible page with a child Ui scoped to
///   that page's rect. Pages are laid out side-by-side horizontally.
pub fn render_sliding_panel(
    ui: &mut egui::Ui,
    page_count: usize,
    offset: f32,
    mut render_page: impl FnMut(&mut egui::Ui, usize),
) -> egui::Response {
    let available = ui.available_size();
    let viewport_width = available.x;

    let (rect, response) = ui.allocate_exact_size(available, egui::Sense::hover());

    // Lay out & render pages side-by-side, translating each by the scroll offset
    for i in 0..page_count {
        let page_x = rect.left() + (i as f32 * viewport_width) - offset;
        let page_rect = egui::Rect::from_min_size(
            egui::pos2(page_x, rect.top()),
            egui::vec2(viewport_width, rect.height()),
        );

        // Skip pages entirely outside the visible viewport
        if page_rect.right() < rect.left() || page_rect.left() > rect.right() {
            continue;
        }

        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(page_rect)
                .layout(egui::Layout::top_down(egui::Align::LEFT)),
        );
        child_ui.set_clip_rect(rect);
        render_page(&mut child_ui, i);
    }

    response
}
