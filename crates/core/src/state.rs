use eframe::egui;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CurrentView {
    Code,
    Command,
}

/// A single open document (tab) in the IDE.
#[derive(Clone)]
pub struct Tab {
    pub text: String,
    /// Snapshot of `text` at the time of the last New/Open/Save/SaveAs.
    pub saved_text: String,
    pub file_path: Option<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
}

impl Tab {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            saved_text: String::new(),
            file_path: None,
            cursor_line: 1,
            cursor_col: 1,
        }
    }

    pub fn with_path_and_content(path: String, content: String) -> Self {
        Self {
            saved_text: content.clone(),
            text: content,
            file_path: Some(path),
            cursor_line: 1,
            cursor_col: 1,
        }
    }

    /// Returns `true` if the text differs from the last saved state.
    pub fn is_modified(&self) -> bool {
        self.text != self.saved_text
    }

    /// Marks the current text as "saved" (clears modified state).
    pub fn mark_saved(&mut self) {
        self.saved_text = self.text.clone();
    }

    /// Returns the file name to display (e.g. "main.rs") or "Untitled".
    pub fn file_name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|p| std::path::Path::new(p).file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Untitled".to_string())
    }
}

/// Includes the cursor line/col, the current code text, the focusing stuff, etc
pub struct AppState {
    /// All open documents.
    pub tabs: Vec<Tab>,
    /// Index into `tabs` of the currently visible tab.
    pub active_tab_index: usize,
    pub editor_focused: bool,
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
            tabs: vec![Tab::new()],
            active_tab_index: 0,
            editor_focused: false,
            editor_id: egui::Id::new("editor"),
            current_view: CurrentView::Code,
            traffic_lights_positioned: false,
            was_fullscreen: false,
            document_edited: false,
        }
    }

    /// Returns a shared reference to the active tab.
    pub fn current_tab(&self) -> &Tab {
        &self.tabs[self.active_tab_index]
    }

    /// Returns a mutable reference to the active tab.
    pub fn current_tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.active_tab_index]
    }

    /// Returns `true` if the active tab's text differs from its last saved state.
    pub fn is_modified(&self) -> bool {
        self.current_tab().is_modified()
    }

    /// Marks the active tab's text as "saved".
    pub fn mark_saved(&mut self) {
        self.current_tab_mut().mark_saved();
    }

    /// Opens a file in a new tab (or switches to it if already open).
    /// Returns the index of the tab.
    pub fn open_file(&mut self, path: String, content: String) -> usize {
        // Check if this file is already open
        for (i, tab) in self.tabs.iter().enumerate() {
            if tab.file_path.as_deref() == Some(&path) {
                self.active_tab_index = i;
                return i;
            }
        }
        // Otherwise create a new tab
        let tab = Tab::with_path_and_content(path, content);
        self.tabs.push(tab);
        let idx = self.tabs.len() - 1;
        self.active_tab_index = idx;
        idx
    }

    /// Adds a new empty tab and returns its index.
    pub fn new_tab(&mut self) -> usize {
        self.tabs.push(Tab::new());
        let idx = self.tabs.len() - 1;
        self.active_tab_index = idx;
        idx
    }

    /// Removes the tab at `index`. If it was the active tab, selects a neighbour.
    /// Will not remove the last remaining tab.
    pub fn close_tab(&mut self, index: usize) {
        if self.tabs.len() <= 1 {
            return;
        }
        self.tabs.remove(index);
        if self.active_tab_index >= self.tabs.len() {
            self.active_tab_index = self.tabs.len() - 1;
        } else if index < self.active_tab_index {
            self.active_tab_index -= 1;
        }
    }

    pub fn switch_to_view(&mut self, target: CurrentView) {
        if target != self.current_view {
            self.current_view = target;
        }
    }
}
