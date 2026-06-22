use eframe::egui::{self, Color32, FontId, TextFormat};
use syntect::easy::HighlightLines;
use syntect::highlighting::{HighlightState, Theme, ThemeSet};
use syntect::parsing::{ParseState, SyntaxReference, SyntaxSet};
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

/// Cached tokenisation result for a single line, including the syntect
/// parser state so we can resume incremental re-highlighting.
#[derive(Clone)]
struct CachedLine {
    /// Line content including its line ending (for change detection).
    content: String,
    /// Highlighted sections with byte offsets **relative to this line**.
    sections: Vec<(usize, usize, Color32)>,
    /// Syntect state *after* this line.
    hl_state: HighlightState,
    parse_state: ParseState,
}

/// A syntax highlighter backed by syntect, producing egui LayoutJobs.
///
/// Highlighting is done **incrementally**: only lines that actually changed
/// are re-tokenised, and re-highlighting stops early once the parser state
/// matches a previously cached line (meaning the rest is identical).
pub struct SyntaxHighlighter {
    font_id: FontId,
    syntax: &'static SyntaxReference,
    theme: &'static Theme,
    lines: Vec<CachedLine>,
}

impl SyntaxHighlighter {
    /// Create a new highlighter.
    ///
    /// `extension` is an optional file extension (e.g. `"rs"`, `"py"`).
    /// If `None` or the extension is not recognised, falls back to plain text.
    pub fn new(font_size: f32, extension: Option<&str>) -> Self {
        let ss = syntax_set();
        let syntax = extension
            .and_then(|ext| ss.find_syntax_by_extension(ext))
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
            lines: Vec::new(),
        }
    }

    /// Highlight source text and return an egui LayoutJob suitable for
    /// use with `TextEdit::layouter`.
    ///
    /// Only lines that actually changed are re-tokenized.  Re-highlighting
    /// also stops early when the parser state stabilises to a previously
    /// cached line, reusing everything after it.
    pub fn highlight(&mut self, text: &str) -> egui::text::LayoutJob {
        if text.is_empty() {
            self.lines.clear();
            return egui::text::LayoutJob::default();
        }

        let ss = syntax_set();
        let new_lines: Vec<&str> = LinesWithEndings::from(text).collect();

        // ── 1. Find the first line whose content differs ──────────
        let first_diff = self
            .lines
            .iter()
            .zip(new_lines.iter())
            .position(|(cached, &new)| cached.content != new)
            .unwrap_or(usize::MAX) // every overlapping line matched
            .min(self.lines.len())
            .min(new_lines.len());

        // Cache hit — text hasn't changed at all
        if first_diff == self.lines.len() && self.lines.len() == new_lines.len() {
            return self.build_job(text);
        }

        // ── 2. Pop old cache from the divergence point ────────────
        let old_remainder: Vec<CachedLine> = self.lines.drain(first_diff..).collect();

        // ── 3. Seed the highlighter from the line before the edit ─
        let mut hl = if first_diff == 0 {
            HighlightLines::new(self.syntax, self.theme)
        } else {
            let prev = &self.lines[first_diff - 1];
            HighlightLines::from_state(
                self.theme,
                prev.hl_state.clone(),
                prev.parse_state.clone(),
            )
        };

        // ── 4. Re-highlight changed/new lines ─────────────────────
        let mut new_cache: Vec<CachedLine> = Vec::new();

        for (rel_idx, &line) in new_lines[first_diff..].iter().enumerate() {
            // Tokenise (errors → empty sections, state still advances)
            let result = hl.highlight_line(line, ss);
            let (hls, ps) = hl.state();

            // Build line-local sections from the token ranges
            let sections = if let Ok(ref ranges) = result {
                ranges
                    .iter()
                    .map(|(style, part)| {
                        let color = Color32::from_rgba_unmultiplied(
                            style.foreground.r,
                            style.foreground.g,
                            style.foreground.b,
                            style.foreground.a,
                        );
                        let part_start = part.as_ptr() as usize - line.as_ptr() as usize;
                        (part_start, part_start + part.len(), color)
                    })
                    .collect()
            } else {
                Vec::new()
            };

            let cached_line = CachedLine {
                content: line.to_string(),
                sections,
                hl_state: hls.clone(),
                parse_state: ps.clone(),
            };

            // ── 5. Early-stop check ───────────────────────────────
            // If the state after this line matches the old cache at the same
            // position AND the remaining text is identical, reuse the rest.
            let should_stop = rel_idx < old_remainder.len()
                && hls == old_remainder[rel_idx].hl_state
                && ps == old_remainder[rel_idx].parse_state
                && new_lines[first_diff + rel_idx + 1..]
                    == old_remainder[rel_idx + 1..]
                        .iter()
                        .map(|c| c.content.as_str())
                        .collect::<Vec<_>>();

            new_cache.push(cached_line);

            if should_stop {
                new_cache.extend(old_remainder[rel_idx + 1..].iter().cloned());
                break;
            }

            // Recreate highlighter for the next line
            hl = HighlightLines::from_state(self.theme, hls, ps);
        }

        // ── 6. Merge into the persistent cache ───────────────────
        self.lines.extend(new_cache);

        // ── 7. Build the LayoutJob ───────────────────────────────
        self.build_job(text)
    }

    // ── helpers ──────────────────────────────────────────────────

    /// Build an egui `LayoutJob` from the current line cache.
    fn build_job(&self, text: &str) -> egui::text::LayoutJob {
        let mut job = egui::text::LayoutJob::default();
        job.text = text.to_owned();

        // Flatten all cached sections into global byte offsets.
        let mut sections: Vec<(usize, usize, Color32)> = Vec::new();
        let mut line_start = 0usize;

        for line in &self.lines {
            for &(start, end, color) in &line.sections {
                sections.push((line_start + start, line_start + end, color));
            }
            line_start += line.content.len();
        }

        // Build LayoutJob sections, filling gaps with the default format.
        let default_fmt = TextFormat::simple(self.font_id.clone(), Color32::BLACK);
        let mut cursor = 0;
        for &(start, end, color) in &sections {
            if start > cursor {
                job.sections.push(egui::text::LayoutSection {
                    leading_space: 0.0,
                    byte_range: cursor..start,
                    format: default_fmt.clone(),
                });
            }
            job.sections.push(egui::text::LayoutSection {
                leading_space: 0.0,
                byte_range: start..end,
                format: TextFormat::simple(self.font_id.clone(), color),
            });
            cursor = end;
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
