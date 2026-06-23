use std::path::PathBuf;

/// Manages the files
pub struct FileManager {
    pub current_path: Option<PathBuf>,
}

impl FileManager {
    pub fn new() -> Self {
        Self { current_path: None }
    }

    /// Opens files dialog and then returns its path
    pub fn open_file_dialog() -> Option<(String, PathBuf)> {
        let file = rfd::FileDialog::new().set_title("Open File").pick_file()?;

        let content = std::fs::read_to_string(&file).ok()?;
        Some((content, file))
    }

    /// Opens save dialog
    pub fn save_as_dialog() -> Option<PathBuf> {
        rfd::FileDialog::new().set_title("Save File").save_file()
    }

    /// Saves content to path
    pub fn save_to_path(content: &str, path: &PathBuf) -> Result<(), std::io::Error> {
        // TODO: Add proper error handling
        std::fs::write(path, content)
    }

    /// Returns display name, like main.rs instead of user/project/main.rs
    pub fn display_name(&self) -> String {
        self.current_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            // Or returns Untitled
            .unwrap_or_else(|| "Untitled".to_string())
    }
}
