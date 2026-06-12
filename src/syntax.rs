use eframe::egui::{self, Color32, FontId, TextFormat};

/// A simple Rust syntax highlighter that produces egui LayoutJob sections.
pub struct SyntaxHighlighter {
    font_id: FontId,
}

/// State for the character-by-character tokenizer.
#[derive(Clone, Copy, PartialEq)]
enum State {
    Normal,
    LineComment,
    BlockComment { depth: u32 },
    StringLiteral,
    CharLiteral,
}

impl SyntaxHighlighter {
    pub fn new(font_size: f32) -> Self {
        Self {
            font_id: FontId::monospace(font_size),
        }
    }

    /// Highlight Rust source text and return a LayoutJob ready for egui.
    pub fn highlight(&self, text: &str) -> egui::text::LayoutJob {
        let mut job = egui::text::LayoutJob::default();

        if text.is_empty() {
            return job;
        }

        let text_bytes = text.as_bytes();
        let sections = self.tokenize(text);

        job.text = text.to_owned();

        let default_format = TextFormat::simple(self.font_id.clone(), Color32::BLACK);

        if sections.is_empty() {
            // Entire text is one normal section
            job.sections.push(egui::text::LayoutSection {
                leading_space: 0.0,
                byte_range: 0..text_bytes.len(),
                format: default_format,
            });
            return job;
        }

        // Fill gaps between tokenized sections with normal formatting
        let mut cursor = 0;
        for &(start, end, ref fmt) in &sections {
            if cursor < start {
                job.sections.push(egui::text::LayoutSection {
                    leading_space: 0.0,
                    byte_range: cursor..start,
                    format: default_format.clone(),
                });
            }
            job.sections.push(egui::text::LayoutSection {
                leading_space: 0.0,
                byte_range: start..end,
                format: fmt.clone(),
            });
            cursor = end;
        }
        if cursor < text_bytes.len() {
            job.sections.push(egui::text::LayoutSection {
                leading_space: 0.0,
                byte_range: cursor..text_bytes.len(),
                format: default_format,
            });
        }

        job
    }

    /// Tokenize Rust source text, returning list of (byte_start, byte_end, TextFormat).
    fn tokenize(&self, text: &str) -> Vec<(usize, usize, TextFormat)> {
        let mut sections: Vec<(usize, usize, TextFormat)> = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        if len == 0 {
            return sections;
        }

        let mut i = 0;
        let mut state = State::Normal;
        let mut token_start = 0;

        while i < len {
            let ch = chars[i];
            match state {
                State::Normal => {
                    // Check for // line comment
                    if ch == '/' && i + 1 < len && chars[i + 1] == '/' {
                        state = State::LineComment;
                        token_start = i;
                        i += 2;
                        continue;
                    }

                    // Check for /* block comment
                    if ch == '/' && i + 1 < len && chars[i + 1] == '*' {
                        state = State::BlockComment { depth: 1 };
                        token_start = i;
                        i += 2;
                        continue;
                    }

                    // Check for string literal
                    if ch == '"' {
                        state = State::StringLiteral;
                        token_start = i;
                        i += 1;
                        continue;
                    }

                    // Check for char literal
                    if ch == '\'' && i + 1 < len
                        && chars[i + 1] != '\''
                        && !chars[i + 1].is_whitespace()
                    {
                        state = State::CharLiteral;
                        token_start = i;
                        i += 1;
                        continue;
                    }

                    // Check if we're at an identifier or number start
                    if ch.is_alphanumeric() || ch == '_' {
                        let word_start = i;
                        while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                            i += 1;
                        }
                        let word: String = chars[word_start..i].iter().collect();

                        let fmt = self.classify_word(&word, &chars, i, len);
                        if let Some(fmt) = fmt {
                            let byte_start = byte_offset(text, word_start);
                            let byte_end = byte_offset(text, i);
                            sections.push((byte_start, byte_end, fmt.clone()));

                            // If this is a macro (followed by !), also consume and highlight the !
                            if i < len && chars[i] == '!' {
                                let bang_start = byte_end;
                                i += 1;
                                let bang_end = byte_offset(text, i);
                                sections.push((bang_start, bang_end, fmt));
                            }
                        }
                        continue;
                    }

                    // Check for lifetime: 'ident
                    if ch == '\'' && i + 1 < len
                        && (chars[i + 1].is_alphabetic() || chars[i + 1] == '_')
                    {
                        let lt_start = i;
                        i += 1;
                        while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                            i += 1;
                        }
                        let byte_start = byte_offset(text, lt_start);
                        let byte_end = byte_offset(text, i);
                        sections.push((
                            byte_start,
                            byte_end,
                            TextFormat::simple(
                                self.font_id.clone(),
                                Color32::from_rgb(0, 128, 128),
                            ),
                        ));
                        continue;
                    }

                    i += 1;
                }

                State::LineComment => {
                    if ch == '\n' {
                        let byte_start = byte_offset(text, token_start);
                        let byte_end = byte_offset(text, i);
                        sections.push((
                            byte_start,
                            byte_end,
                            TextFormat::simple(
                                self.font_id.clone(),
                                Color32::from_rgb(128, 128, 128),
                            ),
                        ));
                        state = State::Normal;
                        i += 1;
                        continue;
                    }
                    i += 1;
                }

                State::BlockComment { depth } => {
                    if ch == '*' && i + 1 < len && chars[i + 1] == '/' {
                        if depth == 1 {
                            let byte_start = byte_offset(text, token_start);
                            let byte_end = byte_offset(text, i + 2);
                            sections.push((
                                byte_start,
                                byte_end,
                                TextFormat::simple(
                                    self.font_id.clone(),
                                    Color32::from_rgb(128, 128, 128),
                                ),
                            ));
                            state = State::Normal;
                            i += 2;
                            continue;
                        }
                    }
                    if ch == '/' && i + 1 < len && chars[i + 1] == '*' {
                        // Nested block comment
                        state = State::BlockComment { depth: depth + 1 };
                        i += 2;
                        continue;
                    }
                    i += 1;
                }

                State::StringLiteral => {
                    if ch == '\\' && i + 1 < len {
                        i += 2; // skip escape sequence
                        continue;
                    }
                    if ch == '"' {
                        let byte_start = byte_offset(text, token_start);
                        let byte_end = byte_offset(text, i + 1);
                        sections.push((
                            byte_start,
                            byte_end,
                            TextFormat::simple(
                                self.font_id.clone(),
                                Color32::from_rgb(0, 128, 0),
                            ),
                        ));
                        state = State::Normal;
                        i += 1;
                        continue;
                    }
                    if ch == '\n' {
                        // Unterminated string - end it at newline
                        let byte_start = byte_offset(text, token_start);
                        let byte_end = byte_offset(text, i);
                        sections.push((
                            byte_start,
                            byte_end,
                            TextFormat::simple(
                                self.font_id.clone(),
                                Color32::from_rgb(0, 128, 0),
                            ),
                        ));
                        state = State::Normal;
                        continue;
                    }
                    i += 1;
                }

                State::CharLiteral => {
                    if ch == '\\' && i + 1 < len {
                        i += 2; // skip escape sequence
                        continue;
                    }
                    if ch == '\'' {
                        let byte_start = byte_offset(text, token_start);
                        let byte_end = byte_offset(text, i + 1);
                        sections.push((
                            byte_start,
                            byte_end,
                            TextFormat::simple(
                                self.font_id.clone(),
                                Color32::from_rgb(0, 128, 0),
                            ),
                        ));
                        state = State::Normal;
                        i += 1;
                        continue;
                    }
                    if ch == '\n' {
                        let byte_start = byte_offset(text, token_start);
                        let byte_end = byte_offset(text, i);
                        sections.push((
                            byte_start,
                            byte_end,
                            TextFormat::simple(
                                self.font_id.clone(),
                                Color32::from_rgb(0, 128, 0),
                            ),
                        ));
                        state = State::Normal;
                        continue;
                    }
                    i += 1;
                }
            }
        }

        // Flush any remaining open state at end-of-file
        match state {
            State::LineComment | State::BlockComment { .. } => {
                let byte_start = byte_offset(text, token_start);
                let byte_end = text.len();
                sections.push((
                    byte_start,
                    byte_end,
                    TextFormat::simple(self.font_id.clone(), Color32::from_rgb(128, 128, 128)),
                ));
            }
            State::StringLiteral | State::CharLiteral => {
                let byte_start = byte_offset(text, token_start);
                let byte_end = text.len();
                sections.push((
                    byte_start,
                    byte_end,
                    TextFormat::simple(self.font_id.clone(), Color32::from_rgb(0, 128, 0)),
                ));
            }
            State::Normal => {}
        }

        sections
    }

    /// Classify a word token and return a TextFormat if it's a special keyword/type/etc.
    fn classify_word(
        &self,
        word: &str,
        chars: &[char],
        pos: usize,
        _len: usize,
    ) -> Option<TextFormat> {
        // Rust keywords
        const KEYWORDS: &[&str] = &[
            "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else",
            "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop",
            "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
            "static", "struct", "super", "trait", "true", "type", "union", "unsafe",
            "use", "where", "while", "yield",
        ];

        // Common Rust types
        const TYPES: &[&str] = &[
            "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize",
            "u8", "u16", "u32", "u64", "u128", "usize", "str", "String", "Vec",
            "Option", "Result", "Box", "Rc", "Arc", "Cell", "RefCell", "HashMap",
            "HashSet", "BTreeMap", "BTreeSet", "Iterator", "impl", "dyn",
        ];

        if KEYWORDS.contains(&word) {
            return Some(TextFormat::simple(
                self.font_id.clone(),
                Color32::from_rgb(175, 0, 200),
            ));
        }

        if TYPES.contains(&word) {
            return Some(TextFormat::simple(
                self.font_id.clone(),
                Color32::from_rgb(0, 100, 180),
            ));
        }

        // Numbers
        if word.chars().all(|c| c.is_ascii_digit() || c == '_')
            || (word.starts_with("0x") && word[2..].chars().all(|c| c.is_ascii_hexdigit() || c == '_'))
            || (word.starts_with("0o") && word[2..].chars().all(|c| c.is_ascii_digit() && c < '8' || c == '_'))
            || (word.starts_with("0b") && word[2..].chars().all(|c| c == '0' || c == '1' || c == '_'))
        {
            return Some(TextFormat::simple(
                self.font_id.clone(),
                Color32::from_rgb(0, 128, 192),
            ));
        }

        // Check if followed by `!` → macro invocation
        if pos < chars.len() && chars[pos] == '!' {
            return Some(TextFormat::simple(
                self.font_id.clone(),
                Color32::from_rgb(180, 80, 180),
            ));
        }

        // Check if followed by `(` → function call
        if pos < chars.len() && chars[pos] == '(' {
            return Some(TextFormat::simple(
                self.font_id.clone(),
                Color32::from_rgb(128, 64, 0),
            ));
        }

        // Check if it starts with uppercase → likely a type/enum
        if word.starts_with(|c: char| c.is_uppercase()) {
            return Some(TextFormat::simple(
                self.font_id.clone(),
                Color32::from_rgb(0, 100, 180),
            ));
        }

        None
    }
}

/// Convert char index to byte offset in a string.
fn byte_offset(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map(|(i, _)| i)
        .unwrap_or(text.len())
}
