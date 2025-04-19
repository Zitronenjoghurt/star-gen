use crate::resources::settings::graphics::GraphicsSettings;
use crate::resources::window_manager::WindowManager;
use bevy::prelude::ResMut;
use bevy_egui::egui;
use bevy_egui::egui::Context;

pub fn draw_graphics_settings_window(
    ctx: &mut Context,
    window_manager: &mut ResMut<WindowManager>,
    graphics_settings: &mut ResMut<GraphicsSettings>,
) {
    egui::Window::new("Graphics Settings")
        .open(&mut window_manager.graphics_settings)
        .show(ctx, |ui| {
            ui.add(egui::Checkbox::new(&mut graphics_settings.vsync, "VSync"));
        });
}
