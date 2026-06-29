use eframe::egui;
use jereide_core::{
    AppState, CurrentView, ITEM_SPACING_Y, TITLE_BAR_HEIGHT, TRAFFIC_LIGHT_OFFSET_X,
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
        self.state.new_tab();
    }

    fn handle_open(&mut self) {
        if let Some((content, path)) = FileManager::open_file_dialog() {
            let path_str = path.display().to_string();
            self.state.open_file(path_str, content);
            self.file_manager.current_path = Some(path);
        }
    }

    fn handle_save(&mut self) {
        let path = self.state.current_tab().file_path.clone();
        match path {
            Some(p) => {
                let text = self.state.current_tab().text.clone();
                if let Err(e) = FileManager::save_to_path(&text, &std::path::PathBuf::from(&p)) {
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
            let text = self.state.current_tab().text.clone();
            if let Err(e) = FileManager::save_to_path(&text, &path) {
                // TODO: Pop out a message thing instead of printing an error
                eprintln!("Failed to save file: {}", e);
            } else {
                let path_str = path.display().to_string();
                let idx = self.state.active_tab_index;
                self.state.tabs[idx].file_path = Some(path_str);
                self.state.mark_saved();
            }
        }
    }

    fn save_tab(&mut self, idx: usize) -> bool {
        let path = self.state.tabs[idx].file_path.clone();
        match path {
            Some(p) => {
                let text = self.state.tabs[idx].text.clone();
                if let Err(e) = FileManager::save_to_path(&text, &std::path::PathBuf::from(&p)) {
                    eprintln!("Failed to save file: {}", e);
                    false
                } else {
                    true
                }
            }
            None => {
                if let Some(path) = FileManager::save_as_dialog() {
                    let text = self.state.tabs[idx].text.clone();
                    if let Err(e) = FileManager::save_to_path(&text, &path) {
                        eprintln!("Failed to save file: {}", e);
                        false
                    } else {
                        let path_str = path.display().to_string();
                        self.state.tabs[idx].file_path = Some(path_str);
                        true
                    }
                } else {
                    false
                }
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
            let is_modified = self.state.is_modified();
            if is_modified != self.state.document_edited {
                self.state.document_edited = is_modified;
                jereide_window::set_document_edited(frame, is_modified);
            }

            let is_fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));

            jereide_window::position_traffic_lights(
                frame,
                TRAFFIC_LIGHT_OFFSET_X,
                TRAFFIC_LIGHT_OFFSET_Y,
            );

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

        {
            let state = &mut self.state;

            jereide_ui::status_bar::render_status_bar(state, ui);

            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(jereide_core::SURFACE_BG))
                .show_inside(ui, |ui| {
                    let style = ui.style_mut();
                    style.visuals.extreme_bg_color = jereide_core::SURFACE_BG;
                    style.spacing.item_spacing.y = ITEM_SPACING_Y;

                    let is_fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
                    jereide_ui::title_bar::render_title_bar(state, ui, is_fullscreen);

                    jereide_ui::tab_strip::render_tab_strip(state, ui);

                    let content_rect = ui.available_rect_before_wrap();
                    let mut code_ui = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(content_rect)
                            .layout(egui::Layout::top_down(egui::Align::LEFT)),
                    );
                    code_ui.set_clip_rect(content_rect);
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

        use jereide_ui::dialog::CloseConfirmAction;

        if let Some(action) = jereide_ui::dialog::render_close_confirm_modal(&mut self.state, &ctx)
        {
            match action {
                CloseConfirmAction::Save(idx) => {
                    if self.save_tab(idx) {
                        self.state.close_tab(idx);
                    }
                    self.state.pending_close_index = None;
                }
                CloseConfirmAction::Discard(idx) => {
                    self.state.close_tab(idx);
                    self.state.pending_close_index = None;
                }
                CloseConfirmAction::Cancel => {
                    self.state.pending_close_index = None;
                }
            }
        }
    }
}
