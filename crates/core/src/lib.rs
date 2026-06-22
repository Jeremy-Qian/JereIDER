pub mod cursor;
pub mod state;
pub mod text_utils;
// Lots of stuff

pub use cursor::char_index_to_line_col;
pub use state::{AppState, CurrentView};
pub use text_utils::{char_range_substring, delete_char_range, insert_at_char_index};
