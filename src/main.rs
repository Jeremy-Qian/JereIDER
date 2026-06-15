use eframe::egui;

mod menu;
mod main_view;
mod status_bar;
mod syntax;
mod text_editor;
mod window_controls;

use menu::AppMenu;

fn main() -> Result<(), eframe::Error> {
    let app_menu = AppMenu::new();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_titlebar_shown(false)
            .with_title_shown(false)
            .with_fullsize_content_view(true),
        ..Default::default()
    };

    eframe::run_native(
        "JereIDE",
        options,
        Box::new(|_cc| Ok(Box::new(JereIDEApp::new(app_menu)))),
    )
}
#[derive(PartialEq)]
enum CurrentView {
    Code,
    Command,
}

struct JereIDEApp {
    code_text: String,
    editor_focused: bool,
    cursor_line: usize,
    cursor_col: usize,
    app_menu: AppMenu,
    editor_id: egui::Id,
    current_view: CurrentView,
    traffic_lights_positioned: bool,
}

impl JereIDEApp {
    fn new(app_menu: AppMenu) -> Self {
        Self {
            code_text: String::new(),
            editor_focused: false,
            cursor_line: 1,
            cursor_col: 1,
            app_menu,
            editor_id: egui::Id::new("editor"),
            current_view: CurrentView::Code,
            traffic_lights_positioned: false,
        }
    }
}

impl eframe::App for JereIDEApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(target_os = "macos")]
        if !self.traffic_lights_positioned {
            window_controls::position_traffic_lights(frame, -3.0);
            self.traffic_lights_positioned = true;
        }

        if !self.app_menu.is_initialized() {
            self.app_menu.init();
            self.app_menu.set_initialized();
        }

        for event_id in self.app_menu.poll_events() {
            match event_id.as_ref() {
                "new" | "open" | "save" => {}
                "quit" => std::process::exit(0),
                "about" => {}
                _ => self.handle_edit_action(ctx, event_id.as_ref()),
            }
        }

        self.render_status_bar(ctx);
        self.render_central_panel(ctx);
    }
}
