use eframe::egui;
use jereide_core::{AppState, CurrentView};
use jereide_fs::FileManager;
use jereide_menu::AppMenu;

pub struct JereIDEApp {
    state: AppState,
    app_menu: AppMenu,
    file_manager: FileManager,
}

impl JereIDEApp {
    pub fn new(app_menu: AppMenu) -> Self {
        Self {
            state: AppState::new(),
            app_menu,
            file_manager: FileManager::new(),
        }
    }

    fn handle_new(&mut self) {
        self.state.code_text.clear();
        self.state.current_file_path = None;
        self.file_manager.current_path = None;
    }

    fn handle_open(&mut self) {
        if let Some((content, path)) = FileManager::open_file_dialog() {
            self.state.code_text = content;
            self.state.current_file_path = Some(path.display().to_string());
            self.file_manager.current_path = Some(path);
        }
    }

    fn handle_save(&mut self) {
        let path = self.file_manager.current_path.clone();
        match path {
            Some(p) => {
                if let Err(e) = FileManager::save_to_path(&self.state.code_text, &p) {
                    eprintln!("Failed to save file: {}", e);
                }
            }
            None => self.handle_save_as(),
        }
    }

    fn handle_save_as(&mut self) {
        if let Some(path) = FileManager::save_as_dialog() {
            if let Err(e) = FileManager::save_to_path(&self.state.code_text, &path) {
                eprintln!("Failed to save file: {}", e);
            } else {
                self.state.current_file_path = Some(path.display().to_string());
                self.file_manager.current_path = Some(path);
            }
        }
    }
}

impl eframe::App for JereIDEApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

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
                "new" => self.handle_new(),
                "open" => self.handle_open(),
                "save" => self.handle_save(),
                "save_as" => self.handle_save_as(),
                "quit" => std::process::exit(0),
                "fullscreen" => {
                    let is_fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!is_fullscreen));
                }
                _ => jereide_code::edit::handle_edit_action(&mut self.state, &ctx, event_id.as_ref()),
            }
        }

        let state = &mut self.state;

        jereide_ui::status_bar::render_status_bar(state, ui);

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(egui::Color32::WHITE))
            .show_inside(ui, |ui| {
                let style = ui.style_mut();
                style.visuals.extreme_bg_color = egui::Color32::WHITE;
                style.spacing.item_spacing.y = 0.0;

                let is_fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
                jereide_ui::title_bar::render_title_bar(state, ui, is_fullscreen);

                let content_rect = ui.available_rect_before_wrap();
                let mut code_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(content_rect)
                        .layout(egui::Layout::top_down(egui::Align::LEFT)),
                );
                jereide_code::code_view::render_code_view(state, &mut code_ui);
            });

        if state.current_view == CurrentView::Command {
            let title_bar_height = 34.0;
            let full_area = ui.ctx().content_rect();
            let overlay_rect = egui::Rect::from_min_size(
                egui::pos2(full_area.left(), full_area.top() + title_bar_height),
                egui::vec2(full_area.width(), full_area.height() - title_bar_height),
            );

            let mut overlay_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(overlay_rect)
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
            );
            jereide_command::command_view::render_command_view(&mut overlay_ui);
        }
    }
}
