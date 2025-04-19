use crate::resources::settings::bloom::BloomSettings;
use crate::resources::settings::wireframe::WireframeSettings;
use crate::resources::window_manager::WindowManager;
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn render_settings_window(
    mut contexts: EguiContexts,
    mut window_manager: ResMut<WindowManager>,
    mut bloom_settings: ResMut<BloomSettings>,
    mut wireframe_settings: ResMut<WireframeSettings>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

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
