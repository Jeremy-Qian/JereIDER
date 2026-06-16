use eframe::egui;

#[derive(PartialEq)]
pub enum CurrentView {
    Code,
    Command,
}

pub struct AppState {
    pub code_text: String,
    pub editor_focused: bool,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub editor_id: egui::Id,
    pub current_view: CurrentView,
    pub traffic_lights_positioned: bool,
    pub was_fullscreen: bool,
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
            traffic_lights_positioned: false,
            was_fullscreen: false,
        }
    }
}
