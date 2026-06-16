use eframe::egui;
use jereide_core::AppState;
use jereide_menu::AppMenu;

struct JereIDEApp {
    state: AppState,
    app_menu: AppMenu,
}

impl JereIDEApp {
    fn new(app_menu: AppMenu) -> Self {
        Self {
            state: AppState::new(),
            app_menu,
        }
    }
}

impl eframe::App for JereIDEApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let ctx = ui.ctx();
        #[cfg(target_os = "macos")]
        {
            let is_fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            if self.state.was_fullscreen != is_fullscreen || !self.state.traffic_lights_positioned {
                jereide_window::position_traffic_lights(frame, 2.0, -3.0);
                self.state.traffic_lights_positioned = true;
            }
            self.state.was_fullscreen = is_fullscreen;
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
                "fullscreen" => {
                    let is_fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!is_fullscreen));
                }
                _ => jereide_ui::edit::handle_edit_action(&mut self.state, ctx, event_id.as_ref()),
            }
        }

        jereide_ui::status_bar::render_status_bar(&self.state, ui);
        jereide_ui::main_view::render_central_panel(&mut self.state, ui);
    }
}

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
        "jereide",
        options,
        Box::new(|_cc| Ok(Box::new(JereIDEApp::new(app_menu)))),
    )
}
