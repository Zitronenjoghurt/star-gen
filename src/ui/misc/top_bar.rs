use crate::resources::window_manager::WindowManager;
use bevy::prelude::ResMut;
use bevy_egui::egui;
use bevy_egui::EguiContexts;

pub fn render_top_bar(mut contexts: EguiContexts, mut window_manager: ResMut<WindowManager>) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Windows", |ui| {
                ui.checkbox(&mut window_manager.cluster_info, "Star Cluster");
                ui.checkbox(&mut window_manager.diagnostics, "Diagnostics");
            });
            ui.menu_button("Generate", |ui| {
                ui.menu_button("Cluster", |ui| {
                    if ui.button("Cubic").clicked() {
                        window_manager.generate_cubic_modal = true;
                    }
                    if ui.button("Seeded").clicked() {
                        window_manager.generate_seeded_modal = true;
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
