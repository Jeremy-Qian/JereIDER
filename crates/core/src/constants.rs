//! Central constants for JereIDE.
//!
//! Colors, dimensions, and other magic numbers that are used in multiple
//! places across the UI.  Keep this file focused on values that don't
//! change at runtime.

use eframe::egui::{Color32, Margin};

// ── Editor colors ────────────────────────────────────────────────────────────

/// Background of the editing area and central panel.
pub const EDITOR_BG: Color32 = Color32::WHITE;

/// Background of the custom title bar.
pub const TITLE_BAR_BG: Color32 = Color32::from_rgb(245, 245, 245);

/// Text color for the filename shown in the title bar.
pub const TITLE_BAR_TEXT: Color32 = Color32::from_rgb(80, 80, 80);

/// Background of the gutter (line-number column).
pub const GUTTER_BG: Color32 = Color32::from_rgb(245, 245, 245);

/// Vertical separator line between the gutter and the code area.
pub const GUTTER_BORDER: Color32 = Color32::from_rgb(224, 224, 224);

/// Background highlight for the current line of code.
pub const CURRENT_LINE_BG: Color32 = Color32::from_rgb(255, 255, 208);

/// Line-number text color for the currently active line.
pub const GUTTER_TEXT_CURRENT: Color32 = Color32::from_rgb(48, 48, 48);

/// Line-number text color for all other lines.
pub const GUTTER_TEXT: Color32 = Color32::from_rgb(145, 145, 145);

/// Default text colour used when syntax highlighting produces no colour.
pub const DEFAULT_TEXT: Color32 = Color32::BLACK;

/// Background of the command-view overlay.
pub const COMMAND_VIEW_BG: Color32 = Color32::from_gray(20);

/// Text colour used in the command-view placeholder.
pub const COMMAND_VIEW_TEXT: Color32 = Color32::from_gray(250);

/// Background of the status bar panel.
pub const STATUS_BAR_BG: Color32 = Color32::WHITE;

// ── Layout dimensions ────────────────────────────────────────────────────────

/// Height of the custom title bar.
pub const TITLE_BAR_HEIGHT: f32 = 34.0;

/// Height of the tab strip.
pub const TAB_STRIP_HEIGHT: f32 = 32.0;

/// Font size for title-bar buttons and labels.
pub const TITLE_BAR_FONT_SIZE: f32 = 12.0;

/// Horizontal space reserved for the native traffic-light buttons
/// (used when the window is **not** fullscreen).
pub const TITLE_BAR_TRAFFIC_SPACE: f32 = 75.0;

/// Horizontal space used when the window **is** fullscreen (no traffic lights).
pub const TITLE_BAR_FULLSCREEN_SPACE: f32 = 7.0;

/// Space inserted before the filename on the right side of the title bar.
pub const TITLE_BAR_FILENAME_SPACE: f32 = 10.0;

/// Vertical gap below a "Choose Project" popup menu button.
pub const TITLE_BAR_POPUP_GAP: f32 = 4.0;

/// Monospace font size for the code editor and syntax highlighter.
pub const EDITOR_FONT_SIZE: f32 = 14.0;

/// Inner margin above the first line of code (inside the TextEdit frame).
pub const EDITOR_INNER_MARGIN_TOP: i8 = 10;

/// Inner margin below the last line of code (inside the TextEdit frame).
pub const EDITOR_INNER_MARGIN_BOTTOM: i8 = 10;

/// Inner margin to the right of the code text.
pub const EDITOR_INNER_MARGIN_RIGHT: i8 = 10;

/// Extra left margin added *on top of* the gutter width.
pub const EDITOR_INNER_MARGIN_LEFT_EXTRA: i8 = 6;

/// Width of a single digit character in the gutter.
pub const GUTTER_DIGIT_WIDTH: f32 = 8.0;

/// Left padding inside the gutter (before the line number).
pub const GUTTER_PADDING_LEFT: f32 = 10.0;

/// Right padding inside the gutter (after the line number).
pub const GUTTER_PADDING_RIGHT: f32 = 6.0;

/// Stroke width of the vertical gutter border line.
pub const GUTTER_BORDER_WIDTH: f32 = 1.0;

/// Horizontal offset from the gutter right edge to the start of the
/// current-line highlight bar.
pub const GUTTER_HIGHLIGHT_OFFSET: f32 = 2.0;

/// Rightward offset for the line-number text within the gutter,
/// measured from the gutter's right edge.
pub const GUTTER_LINE_NUMBER_RIGHT_OFFSET: f32 = 5.0;

/// Width of the scroll bar.
pub const SCROLL_BAR_WIDTH: f32 = 12.0;

/// Inner margin of the status bar (horizontal, vertical).
pub const STATUS_BAR_MARGIN: Margin = Margin::symmetric(8, 4);

// ── Window ───────────────────────────────────────────────────────────────────

/// Default inner width of the application window.
pub const WINDOW_WIDTH: f32 = 800.0;

/// Default inner height of the application window.
pub const WINDOW_HEIGHT: f32 = 600.0;

/// X-offset applied to the native traffic-light buttons (positive = right).
pub const TRAFFIC_LIGHT_OFFSET_X: f64 = 2.0;

/// Y-offset applied to the native traffic-light buttons (positive = down).
pub const TRAFFIC_LIGHT_OFFSET_Y: f64 = -3.0;
