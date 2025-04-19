use crate::resources::simulation_settings::{BloomSettings, WireframeSettings};
use crate::resources::window_manager::WindowManager;
use bevy::prelude::ResMut;
use bevy_egui::egui;
use bevy_egui::egui::Context;

pub fn draw_settings_window(
    ctx: &mut Context,
    window_manager: &mut ResMut<WindowManager>,
    bloom_settings: &mut ResMut<BloomSettings>,
    wireframe_settings: &mut ResMut<WireframeSettings>,
) {
    egui::Window::new("Settings")
        .open(&mut window_manager.settings)
        .show(ctx, |ui| {
            ui.add(egui::Checkbox::new(&mut bloom_settings.active, "Bloom"));
            ui.add(egui::Checkbox::new(
                &mut wireframe_settings.active,
                "Wireframe",
            ));
        });
}
