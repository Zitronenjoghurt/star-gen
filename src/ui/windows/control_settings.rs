use crate::resources::settings::controls::ControlSettings;
use crate::resources::window_manager::WindowManager;
use crate::ui::elements::settings_slider::SettingsSlider;
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn render_control_settings_window(
    mut contexts: EguiContexts,
    mut window_manager: ResMut<WindowManager>,
    mut control_settings: ResMut<ControlSettings>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    egui::Window::new("Control Settings")
        .open(&mut window_manager.control_settings)
        .resizable(false)
        .show(ctx, |ui| {
            SettingsSlider::build()
                .text("Star focus auto-zoom factor")
                .tooltip(
                    "Determines how far away the camera stops zooming into a star on selection.",
                )
                .draw(
                    ui,
                    &mut control_settings.star_focus_auto_zoom_factor,
                    ControlSettings::DEFAULT_STAR_FOCUS_AUTO_ZOOM_FACTOR,
                    1.0..=20.0,
                    0.01,
                );

            ui.vertical_centered(|ui| {
                ui.small("Hover over the slider labels for more info");
            });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                if ui.button("Reset").clicked() {
                    control_settings.reset();
                }
            });
        });
}
