use crate::resources::settings::graphics::GraphicsSettings;
use crate::resources::window_manager::WindowManager;
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn render_graphics_settings_window(
    mut contexts: EguiContexts,
    mut window_manager: ResMut<WindowManager>,
    mut graphics_settings: ResMut<GraphicsSettings>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    egui::Window::new("Graphics Settings")
        .open(&mut window_manager.graphics_settings)
        .show(ctx, |ui| {
            ui.add(egui::Checkbox::new(&mut graphics_settings.vsync, "VSync"));
        });
}
