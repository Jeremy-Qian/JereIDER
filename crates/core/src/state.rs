use eframe::egui;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CurrentView {
    Code,
    Command,
}

/// Includes the cursor line/col, the current code text, the focusing stuff, etc
pub struct AppState {
    pub code_text: String,
    /// The text content at the time of the last New/Open/Save/SaveAs.
    /// Compared against `code_text` to determine if the file is modified.
    pub saved_text: String,
    pub current_file_path: Option<String>,
    pub editor_focused: bool,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub editor_id: egui::Id,
    pub current_view: CurrentView,
    pub traffic_lights_positioned: bool,
    pub was_fullscreen: bool,
    /// Tracks the last value sent to `setDocumentEdited:` so we avoid
    /// spamming AppKit on every frame (which triggers unwanted title-bar
    /// re-layout and resets the traffic light positions).
    pub document_edited: bool,
}

/// Another new method.
impl AppState {
    pub fn new() -> Self {
        Self {
            code_text: String::new(),
            saved_text: String::new(),
            current_file_path: None,
            editor_focused: false,
            cursor_line: 1,
            cursor_col: 1,
            editor_id: egui::Id::new("editor"),
            current_view: CurrentView::Code,
            traffic_lights_positioned: false,
            was_fullscreen: false,
            document_edited: false,
        }
    }

    /// Returns `true` if the current text differs from the last saved state.
    pub fn is_modified(&self) -> bool {
        self.code_text != self.saved_text
    }

    /// Marks the current text as "saved" (clears the modified state).
    pub fn mark_saved(&mut self) {
        self.saved_text = self.code_text.clone();
    }

    pub fn switch_to_view(&mut self, target: CurrentView) {
        if target != self.current_view {
            self.current_view = target;
        }
    }
}
