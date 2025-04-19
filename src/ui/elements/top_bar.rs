use crate::resources::window_manager::WindowManager;
use bevy::prelude::ResMut;
use bevy_egui::egui;
use bevy_egui::egui::Context;

pub fn draw_top_bar(ctx: &mut Context, window_manager: &mut ResMut<WindowManager>) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Windows", |ui| {
                ui.checkbox(&mut window_manager.diagnostics, "Diagnostics");
            });
            ui.menu_button("Generate", |ui| {
                ui.menu_button("Cluster", |ui| {
                    if ui.button("Cubic").clicked() {
                        window_manager.generate_cubic_modal = true;
                    }
                });
            });
            ui.menu_button("Settings", |ui| {
                ui.checkbox(&mut window_manager.settings, "General");
                ui.checkbox(&mut window_manager.graphics_settings, "Graphics");
                ui.menu_button("Advanced", |ui| {
                    ui.checkbox(&mut window_manager.bloom_settings, "Bloom");
                });
            });
        });
    });
}
