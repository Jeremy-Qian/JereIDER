//! Central constants for JereIDE.
//!
//! Includes stuff like colors, dimensions, and other constants.
// TODO: Change all of these to variables soon
// TODO: Load all of these from a settings.json or something
// like that at runtime instead of hardcoding them.

use eframe::egui::{Color32, Margin};

pub const EDITOR_BG: Color32 = Color32::WHITE;
pub const TITLE_BAR_BG: Color32 = Color32::from_rgb(245, 245, 245);
pub const GUTTER_BG: Color32 = Color32::WHITE;
pub const GUTTER_TEXT_CURRENT: Color32 = Color32::from_rgb(48, 48, 48);
pub const GUTTER_TEXT: Color32 = Color32::from_rgb(145, 145, 145);
pub const DEFAULT_TEXT: Color32 = Color32::BLACK;
pub const COMMAND_VIEW_BG: Color32 = Color32::from_gray(20);
pub const COMMAND_VIEW_TEXT: Color32 = Color32::from_gray(250);
pub const STATUS_BAR_BG: Color32 = Color32::WHITE;

pub const TAB_STRIP_BG: Color32 = Color32::from_rgb(245, 245, 245);
pub const TAB_ACTIVE_BG: Color32 = Color32::from_rgb(255, 255, 255);
pub const TAB_INACTIVE_BG: Color32 = Color32::from_rgb(245, 245, 245);
pub const TAB_ACTIVE_TEXT: Color32 = Color32::from_rgb(30, 30, 30);
pub const TAB_INACTIVE_TEXT: Color32 = Color32::from_rgb(130, 130, 130);
pub const TAB_BORDER: Color32 = Color32::from_rgb(200, 200, 200);
pub const TAB_CLOSE_BG_HOVER: Color32 = Color32::from_rgb(230, 230, 230);
pub const TAB_CLOSE_ICON: Color32 = Color32::from_rgb(30, 30, 30);
pub const TAB_CLOSE_ICON_HOVER: Color32 = Color32::BLACK;
pub const TAB_MODIFIED_DOT: Color32 = Color32::from_rgb(9, 105, 218);

pub const TITLE_BAR_HEIGHT: f32 = 34.0;
pub const TAB_STRIP_HEIGHT: f32 = 30.0;
pub const TITLE_BAR_FONT_SIZE: f32 = 12.0;
pub const TITLE_BAR_TRAFFIC_SPACE: f32 = 75.0;
pub const TITLE_BAR_FULLSCREEN_SPACE: f32 = 7.0;
pub const TITLE_BAR_POPUP_GAP: f32 = 4.0;
pub const EDITOR_FONT_SIZE: f32 = 14.0;
pub const EDITOR_INNER_MARGIN_TOP: i8 = 10;
pub const EDITOR_INNER_MARGIN_BOTTOM: i8 = 10;
pub const EDITOR_INNER_MARGIN_RIGHT: i8 = 10;
pub const EDITOR_INNER_MARGIN_LEFT_EXTRA: i8 = 6;
pub const GUTTER_DIGIT_WIDTH: f32 = 12.0;
pub const GUTTER_PADDING_LEFT: f32 = 10.0;
pub const GUTTER_PADDING_RIGHT: f32 = 6.0;
pub const GUTTER_LINE_NUMBER_RIGHT_OFFSET: f32 = 5.0;
pub const SCROLL_BAR_WIDTH: f32 = 12.0;
pub const STATUS_BAR_MARGIN: Margin = Margin::symmetric(8, 4);

pub const TAB_PAD_LEFT: f32 = 8.0;
pub const TAB_PAD_RIGHT: f32 = 8.0;
pub const TAB_CLOSE_BTN_SIZE: f32 = 18.0;
pub const TAB_CLOSE_BTN_RADIUS: u8 = 3;
pub const TAB_CLOSE_BTN_SPACING: f32 = 6.0;
pub const TAB_CLOSE_ICON_HALF: f32 = 3.0;
pub const TAB_CLOSE_STROKE: f32 = 0.9;
pub const TAB_MODIFIED_DOT_RADIUS: f32 = 3.0;
pub const TAB_BORDER_WIDTH: f32 = 1.0;
pub const TAB_FONT_SIZE: f32 = 12.0;
pub const SAVE_BUTTON_BG: Color32 = Color32::from_rgb(29, 205, 218);

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const COMMAND_VIEW_FONT_SIZE: f32 = 18.0;
pub const ITEM_SPACING_Y: f32 = 0.0;

pub const TRAFFIC_LIGHT_OFFSET_X: f64 = 2.0;
pub const TRAFFIC_LIGHT_OFFSET_Y: f64 = -3.0;
