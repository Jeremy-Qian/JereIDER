mod menu;

use eframe::egui;
use menu::AppMenu;

fn main() -> Result<(), eframe::Error> {
    // Initialize the native menu (macOS menu bar) before the window.
    // We keep it alive by storing it in the app state.
    // The menu must be created before or shortly after the window.
    let app_menu = AppMenu::new();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Monaco Text Editor"),
        ..Default::default()
    };

    eframe::run_native(
        "Minimal Editor",
        options,
        Box::new(|_cc| Ok(Box::new(EditorApp::new(app_menu)))),
    )
}

struct EditorApp {
    code_text: String,
    editor_focused: bool,
    cursor_line: usize,
    cursor_col: usize,
    app_menu: AppMenu,
}

impl EditorApp {
    fn new(app_menu: AppMenu) -> Self {
        Self {
            code_text: String::new(),
            editor_focused: false,
            cursor_line: 1,
            cursor_col: 1,
            app_menu,
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Init native menu once after the window/event loop is up
        if !self.app_menu.is_initialized() {
            self.app_menu.init();
            self.app_menu.set_initialized();
        }

        // Poll and handle native menu events
        for event_id in self.app_menu.poll_events() {
            match event_id.as_ref() {
                "new" => println!("New file"),
                "open" => println!("Open file"),
                "save" => println!("Save file"),
                "quit" => std::process::exit(0),
                "undo" => println!("Undo"),
                "redo" => println!("Redo"),
                "select_all" => println!("Select All"),
                _ => {}
            }
        }

        egui::TopBottomPanel::bottom("status_bar")
            .frame(
                egui::Frame::NONE
                    .fill(egui::Color32::WHITE)
                    .inner_margin(egui::Margin::symmetric(8, 4)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Ready");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("Ln {}, Col {}", self.cursor_line, self.cursor_col));
                    });
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(egui::Color32::WHITE))
            .show(ctx, |ui| {
                let available = ui.available_size();

                let style = ui.style_mut();
                style.visuals.extreme_bg_color = egui::Color32::WHITE;
                style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
                style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
                style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
                style.spacing.scroll = {
                    let mut s = egui::style::ScrollStyle::solid();
                    s.bar_width = 12.0;
                    s
                };

                let output = egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        ui.add_sized(
                            available,
                            egui::TextEdit::multiline(&mut self.code_text)
                                .id_source("editor")
                                .font(egui::TextStyle::Monospace)
                                .frame(false)
                                .margin(5)
                                .text_color(egui::Color32::BLACK),
                        )
                    });

                let response = output.inner;
                if let Some(state) = egui::TextEdit::load_state(ctx, response.id) {
                    if let Some(range) = state.cursor.char_range() {
                        let (line, col) =
                            char_index_to_line_col(&self.code_text, range.primary.index);
                        self.cursor_line = line + 1;
                        self.cursor_col = col + 1;
                    }
                }

                if !self.editor_focused {
                    self.editor_focused = true;
                    response.request_focus();
                }
            });
    }
}

fn char_index_to_line_col(text: &str, char_index: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;
    for (ci, ch) in text.chars().enumerate() {
        if ci >= char_index {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}
