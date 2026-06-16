/// Extract a substring by character range.
pub fn char_range_substring(text: &str, start_char: usize, end_char: usize) -> String {
    text.chars()
        .skip(start_char)
        .take(end_char - start_char)
        .collect()
}

/// Delete a character range from a string.
pub fn delete_char_range(text: &str, start_char: usize, end_char: usize) -> String {
    text.chars()
        .enumerate()
        .filter(|(i, _)| *i < start_char || *i >= end_char)
        .map(|(_, c)| c)
        .collect()
}

/// Insert a string at a specific character index.
pub fn insert_at_char_index(text: &str, char_index: usize, insert: &str) -> String {
    let before: String = text.chars().take(char_index).collect();
    let after: String = text.chars().skip(char_index).collect();
    format!("{}{}{}", before, insert, after)
}
