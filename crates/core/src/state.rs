use eframe::egui;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CurrentView {
    Code,
    Command,
}

impl CurrentView {
    pub fn index(&self) -> usize {
        match self {
            CurrentView::Code => 0,
            CurrentView::Command => 1,
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => CurrentView::Code,
            _ => CurrentView::Command,
        }
    }
}

pub struct AppState {
    pub code_text: String,
    pub editor_focused: bool,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub editor_id: egui::Id,
    pub current_view: CurrentView,
    pub previous_view: CurrentView,
    pub slide_t: f32,
    pub traffic_lights_positioned: bool,
    pub was_fullscreen: bool,
    pub last_frame_time: Option<f64>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            code_text: String::new(),
            editor_focused: false,
            cursor_line: 1,
            cursor_col: 1,
            editor_id: egui::Id::new("editor"),
            current_view: CurrentView::Code,
            previous_view: CurrentView::Code,
            slide_t: 1.0,
            traffic_lights_positioned: false,
            was_fullscreen: false,
            last_frame_time: None,
        }
    }

    /// Returns the visual slide offset in pixels for the given viewport width.
    /// Uses the same OutCubic easing as the Python SlidingPanel (300ms).
    pub fn slide_offset(&self, viewport_width: f32) -> f32 {
        if self.slide_t >= 1.0 {
            return self.current_view.index() as f32 * viewport_width;
        }
        let t = 1.0 - (1.0 - self.slide_t).powf(3.0); // OutCubic
        let from = self.previous_view.index() as f32 * viewport_width;
        let to = self.current_view.index() as f32 * viewport_width;
        egui::lerp(from..=to, t)
    }

    /// Request a slide from the current view to `target`.
    pub fn slide_to_view(&mut self, target: CurrentView) {
        if target == self.current_view {
            return;
        }
        self.previous_view = self.current_view;
        self.current_view = target;
        self.slide_t = 0.0;
    }
}
