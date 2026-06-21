use eframe::egui;
use jereide_core::AppState;
use jereide_menu::AppMenu;

pub struct JereIDEApp {
    state: AppState,
    app_menu: AppMenu,
}

impl JereIDEApp {
    pub fn new(app_menu: AppMenu) -> Self {
        Self {
            state: AppState::new(),
            app_menu,
        }
    }
}

impl eframe::App for JereIDEApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // --- Delta time for smooth animation ---
        let now = ui.input(|i| i.time);
        let dt = if let Some(last) = self.state.last_frame_time {
            (now - last) as f32
        } else {
            0.0
        };
        self.state.last_frame_time = Some(now);

        // --- Advance slide animation (300ms OutCubic) ---
        if self.state.slide_t < 1.0 {
            self.state.slide_t = (self.state.slide_t + dt / 0.3).min(1.0);
            ctx.request_repaint();
        }

        // --- macOS traffic lights ---
        #[cfg(target_os = "macos")]
        {
            let is_fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            if self.state.was_fullscreen != is_fullscreen || !self.state.traffic_lights_positioned {
                jereide_window::position_traffic_lights(frame, 2.0, -3.0);
                self.state.traffic_lights_positioned = true;
            }
            self.state.was_fullscreen = is_fullscreen;
        }

        // --- Menu init & event handling ---
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
                _ => jereide_code::edit::handle_edit_action(&mut self.state, &ctx, event_id.as_ref()),
            }
        }

        // --- Main UI: title bar + sliding panel + status bar ---
        let state = &mut self.state;
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(egui::Color32::WHITE))
            .show_inside(ui, |ui| {
                let style = ui.style_mut();
                style.visuals.extreme_bg_color = egui::Color32::WHITE;
                style.spacing.item_spacing.y = 0.0;

                let is_fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
                jereide_ui::title_bar::render_title_bar(state, ui, is_fullscreen);

                // Sliding panel with Code view (page 0) and Command view (page 1)
                let available = ui.available_size();
                let viewport_width = available.x;
                let offset = state.slide_offset(viewport_width);

                jereide_ui::sliding_panel::render_sliding_panel(
                    ui,
                    2,
                    offset,
                    |ui, page_idx| {
                        match page_idx {
                            0 => jereide_code::code_view::render_code_view(state, ui),
                            1 => {
                                jereide_command::command_view::render_command_view(state, ui)
                            }
                            _ => {}
						}
                    },
                );

                jereide_ui::status_bar::render_status_bar(state, ui);
            });
    }
}
