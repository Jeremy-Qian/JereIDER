use eframe::egui;
use jereide_core::{
    AppState, CurrentView, TITLE_BAR_HEIGHT, TRAFFIC_LIGHT_OFFSET_X,
    TRAFFIC_LIGHT_OFFSET_Y,
};
use jereide_fs::FileManager;
use jereide_menu::AppMenu;

/// Contains the app state, the menu, and the file manager.

pub struct JereIDEApp {
    state: AppState,
    app_menu: AppMenu,
    file_manager: FileManager,
}

/// Just defaults for it.

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
        self.state.mark_saved();
    }

    fn handle_open(&mut self) {
        if let Some((content, path)) = FileManager::open_file_dialog() {
            self.state.code_text = content;
            self.state.current_file_path = Some(path.display().to_string());
            self.file_manager.current_path = Some(path);
            self.state.mark_saved();
        }
    }

    fn handle_save(&mut self) {
        let path = self.file_manager.current_path.clone();
        match path {
            Some(p) => {
                if let Err(e) = FileManager::save_to_path(&self.state.code_text, &p) {
                    // TODO: Pop out a message thing instead of printing an error
                    eprintln!("Failed to save file: {}", e);
                } else {
                    self.state.mark_saved();
                }
            }
            None => self.handle_save_as(),
        }
    }

    fn handle_save_as(&mut self) {
        if let Some(path) = FileManager::save_as_dialog() {
            if let Err(e) = FileManager::save_to_path(&self.state.code_text, &path) {
                // TODO: Pop out a message thing instead of printing an error
                eprintln!("Failed to save file: {}", e);
            } else {
                self.state.current_file_path = Some(path.display().to_string());
                self.file_manager.current_path = Some(path);
                self.state.mark_saved();
            }
        }
    }
}

impl eframe::App for JereIDEApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        // Actually the only target is macOS so far, Windows support is planned
        #[cfg(target_os = "macos")]
        {
            // Sync the native close-button dirty dot, but only when the value
            // actually changes — AppKit re-lays out the title bar in response to
            // setDocumentEdited:, and doing it every frame would keep resetting
            // our custom traffic light positions.
            let is_modified = self.state.is_modified();
            if is_modified != self.state.document_edited {
                self.state.document_edited = is_modified;
                jereide_window::set_document_edited(frame, is_modified);
                // AppKit may re-lay out the title bar in response to
                // setDocumentEdited:, so force re-positioning of the
                // traffic lights on this frame.
                self.state.traffic_lights_positioned = false;
            }

            let is_fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            if self.state.was_fullscreen != is_fullscreen || !self.state.traffic_lights_positioned {
                // Position the traffic lights like how Zed does it
                jereide_window::position_traffic_lights(frame, TRAFFIC_LIGHT_OFFSET_X, TRAFFIC_LIGHT_OFFSET_Y);
                self.state.traffic_lights_positioned = true;
            }
            self.state.was_fullscreen = is_fullscreen;
        }

        if !self.app_menu.is_initialized() {
            self.app_menu.init();
            self.app_menu.set_initialized();
        }

        for event_id in self.app_menu.poll_events() {
            // The menu actions
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
                _ => {
                    jereide_code::edit::handle_edit_action(&mut self.state, &ctx, event_id.as_ref())
                }
            }
        }

        let state = &mut self.state;

        jereide_ui::status_bar::render_status_bar(state, ui);

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(jereide_core::EDITOR_BG))
            .show_inside(ui, |ui| {
                let style = ui.style_mut();
                style.visuals.extreme_bg_color = jereide_core::EDITOR_BG;
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

        // Command view covers everything
        if state.current_view == CurrentView::Command {
            let title_bar_height = TITLE_BAR_HEIGHT;
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
