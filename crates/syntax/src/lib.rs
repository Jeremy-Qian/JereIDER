use eframe::egui::{self, Color32, FontId, TextFormat};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;
use std::sync::OnceLock;

fn syntax_set() -> &'static SyntaxSet {
    static SS: OnceLock<SyntaxSet> = OnceLock::new();
    SS.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn theme_set() -> &'static ThemeSet {
    static TS: OnceLock<ThemeSet> = OnceLock::new();
    TS.get_or_init(ThemeSet::load_defaults)
}

/// A syntax highlighter backed by syntect, producing egui LayoutJobs.
pub struct SyntaxHighlighter {
    font_id: FontId,
    syntax: &'static SyntaxReference,
    theme: &'static Theme,
}

impl SyntaxHighlighter {
    pub fn new(font_size: f32) -> Self {
        let ss = syntax_set();
        let syntax = ss
            .find_syntax_by_extension("rs")
            .unwrap_or_else(|| ss.find_syntax_plain_text());

        let ts = theme_set();
        // Pick a light theme since our editor background is white.
        let theme = ts
            .themes
            .get("InspiredGitHub")
            .unwrap_or_else(|| &ts.themes["base16-ocean.light"]);

        Self {
            font_id: FontId::monospace(font_size),
            syntax,
            theme,
        }
    }

    /// Highlight source text and return an egui LayoutJob suitable for
    /// use with `TextEdit::layouter`.
    pub fn highlight(&self, text: &str) -> egui::text::LayoutJob {
        let mut job = egui::text::LayoutJob::default();
        if text.is_empty() {
            return job;
        }

        job.text = text.to_owned();

        // Tokenise with syntect
        let ss = syntax_set();
        let mut highlighter = HighlightLines::new(self.syntax, self.theme);
        let mut sections: Vec<(usize, usize, Color32)> = Vec::new();
        let mut byte_cursor = 0;

        for line in LinesWithEndings::from(text) {
            let Ok(ranges) = highlighter.highlight_line(line, ss) else {
                byte_cursor += line.len();
                continue;
            };
            let line_start_in_text = byte_cursor;
            for (style, part) in ranges {
                let fg = style.foreground;
                let color = Color32::from_rgba_unmultiplied(fg.r, fg.g, fg.b, fg.a);
                let part_start =
                    line_start_in_text + (part.as_ptr() as usize - line.as_ptr() as usize);
                let part_end = part_start + part.len();
                sections.push((part_start, part_end, color));
            }
            byte_cursor += line.len();
        }

        // Build the LayoutJob sections from the tokenised regions,
        // filling gaps with the default (black) format.
        let default_fmt = TextFormat::simple(self.font_id.clone(), Color32::BLACK);
        let mut cursor = 0;
        for (start, end, color) in &sections {
            if *start > cursor {
                job.sections.push(egui::text::LayoutSection {
                    leading_space: 0.0,
                    byte_range: cursor..*start,
                    format: default_fmt.clone(),
                });
            }
            job.sections.push(egui::text::LayoutSection {
                leading_space: 0.0,
                byte_range: *start..*end,
                format: TextFormat::simple(self.font_id.clone(), *color),
            });
            cursor = *end;
        }
        if cursor < text.len() {
            job.sections.push(egui::text::LayoutSection {
                leading_space: 0.0,
                byte_range: cursor..text.len(),
                format: default_fmt,
            });
        }

        job
    }
}
