use crate::resources::simulation_settings::{BloomSettings, WireframeSettings};
use crate::resources::window_manager::WindowManager;
use bevy::prelude::ResMut;
use bevy_egui::egui;
use bevy_egui::egui::Context;

pub fn draw_top_bar(
    ctx: &mut Context,
    window_manager: &mut ResMut<WindowManager>,
    bloom_settings: &mut ResMut<BloomSettings>,
    wireframe_settings: &mut ResMut<WireframeSettings>,
) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Windows", |ui| {
                ui.checkbox(&mut window_manager.settings, "Settings");
                ui.checkbox(&mut window_manager.diagnostics, "Diagnostics");
            });
            ui.menu_button("Settings", |ui| {
                ui.checkbox(&mut bloom_settings.active, "Bloom Effect");
                ui.checkbox(&mut wireframe_settings.active, "Wireframe Mode");
            });
        });
    });
}
