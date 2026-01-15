use eframe::egui::{CentralPanel, Context};
use eframe::{App, Frame, Storage};

use crate::config::Settings;
use crate::mouse::MouseController;
use crate::ui::{labeled_slider, progress_bar, status_label};

const SETTINGS_KEY: &str = "mouse_jiggler_settings";

// UI constants
const JIGGLE_DISPLAY_MS: u64 = 500;
const PROGRESS_BAR_WIDTH: usize = 10;
const REPAINT_INTERVAL_MS: u64 = 100;

// Emoticons
const EMOTICON_JIGGLING: &str = "  ~( ^.^ )~  ";
const EMOTICON_WAITING: &str = "  ( -.- )   ";

pub struct MouseJigglerApp {
    is_running: bool,
    show_settings: bool,
    settings: Settings,
    mouse_controller: MouseController,
}

impl MouseJigglerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings = cc
            .storage
            .and_then(|s| {
                s.get_string(SETTINGS_KEY)
                    .and_then(|json| serde_json::from_str(&json).ok())
            })
            .unwrap_or_default();

        Self {
            is_running: false,
            show_settings: false,
            settings,
            mouse_controller: MouseController::new(),
        }
    }

    fn toggle_running(&mut self) {
        if self.is_running {
            self.mouse_controller.stop();
        } else {
            self.mouse_controller.start(&self.settings);
        }
        self.is_running = !self.is_running;
    }

    fn render_main_view(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Mouse Jiggler");
        ui.add_space(10.0);

        self.render_toggle_button(ui);

        if self.is_running {
            ui.add_space(10.0);
            self.render_status(ui);
        }

        ui.add_space(10.0);
        if ui.button("Settings").clicked() {
            self.show_settings = true;
        }
    }

    fn render_toggle_button(&mut self, ui: &mut eframe::egui::Ui) {
        let button_text = if self.is_running {
            "Stop"
        } else {
            "Emulate Mouse"
        };

        if ui.button(button_text).clicked() {
            self.toggle_running();
        }
    }

    fn render_status(&self, ui: &mut eframe::egui::Ui) {
        let ms_since = self.mouse_controller.ms_since_last_jiggle();
        let interval_ms = self.settings.interval_secs * 1000;

        if ms_since < JIGGLE_DISPLAY_MS {
            status_label(ui, EMOTICON_JIGGLING, Some("   JIGGLE!   "));
        } else {
            let remaining_secs = interval_ms.saturating_sub(ms_since) / 1000;
            let progress = if interval_ms > 0 {
                (ms_since as f32 / interval_ms as f32).min(1.0)
            } else {
                0.0
            };

            let bar = progress_bar(progress, PROGRESS_BAR_WIDTH);
            let message = format!("Next: {}s {}", remaining_secs, bar);
            status_label(ui, EMOTICON_WAITING, Some(&message));
        }
    }

    fn render_settings_view(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Settings");
        ui.add_space(10.0);

        labeled_slider(ui, "Interval (seconds):", &mut self.settings.interval_secs, 1..=120);
        ui.add_space(5.0);

        labeled_slider(ui, "Delay (ms):", &mut self.settings.delay_ms, 10..=1000);
        ui.add_space(5.0);

        labeled_slider(ui, "Distance (px):", &mut self.settings.distance, 1..=100);
        ui.add_space(15.0);

        if ui.button("Back").clicked() {
            self.show_settings = false;
        }
    }
}

impl App for MouseJigglerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.is_running {
            ctx.request_repaint_after(std::time::Duration::from_millis(REPAINT_INTERVAL_MS));
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                if self.show_settings {
                    self.render_settings_view(ui);
                } else {
                    self.render_main_view(ui);
                }
            });
        });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        if let Ok(json) = serde_json::to_string(&self.settings) {
            storage.set_string(SETTINGS_KEY, json);
        }
    }
}
