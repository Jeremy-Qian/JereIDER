pub mod state;
pub mod cursor;
pub mod text_utils;

pub use state::{AppState, CurrentView};
pub use cursor::char_index_to_line_col;
pub use text_utils::{char_range_substring, delete_char_range, insert_at_char_index};
