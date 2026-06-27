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


}
