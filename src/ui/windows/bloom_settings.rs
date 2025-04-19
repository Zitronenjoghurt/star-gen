use crate::resources::settings::bloom::BloomSettings;
use crate::resources::window_manager::WindowManager;
use crate::ui::elements::settings_slider::draw_settings_slider;
use bevy::prelude::ResMut;
use bevy_egui::egui;
use bevy_egui::egui::Context;

pub fn draw_bloom_settings_window(
    ctx: &mut Context,
    window_manager: &mut ResMut<WindowManager>,
    bloom_settings: &mut ResMut<BloomSettings>,
) {
    egui::Window::new("Bloom Settings")
        .open(&mut window_manager.bloom_settings)
        .resizable(false)
        .show(ctx, |ui| {
            ui.add(egui::Checkbox::new(&mut bloom_settings.active, "Active"));
            draw_settings_slider(
                ui,
                "Intensity",
                &mut bloom_settings.intensity,
                BloomSettings::DEFAULT_INTENSITY,
                0.0..=2.0,
                0.01,
                "Represents how much scattered light is added to the image to create the glow effect.\n\n• 0.0 means no bloom\n• Greater than 0.0 means a proportionate amount of scattered light is added",
            );
            draw_settings_slider(
                ui,
                "Low frequency boost",
                &mut bloom_settings.low_frequency_boost,
                BloomSettings::DEFAULT_LOW_FREQUENCY_BOOST,
                0.0..=1.0,
                0.01,
                "Controls how much more likely the light is to scatter completely sideways (low frequency image).",
            );
            draw_settings_slider(
                ui,
                "Low frequency boost curvature",
                &mut bloom_settings.low_frequency_boost_curvature,
                BloomSettings::DEFAULT_LOW_FREQUENCY_BOOST_CURVATURE,
                0.0..=1.0,
                0.01,
                "Controls the curvature of the blend factor function making frequencies next to the lowest ones contribute more.\n\n• 0.0 - base intensity and boosted intensity are linearly interpolated\n• 1.0 - all frequencies below maximum are at boosted intensity level",
            );
            draw_settings_slider(
                ui,
                "High pass frequency",
                &mut bloom_settings.high_pass_frequency,
                BloomSettings::DEFAULT_HIGH_PASS_FREQUENCY,
                0.0..=1.0,
                0.01,
                "Tightens how much the light scatters.\n\n• maximum scattering angle is 0 degrees (no scattering)\n• maximum scattering angle is 90 degrees",
            );
            draw_settings_slider(
                ui,
                "Prefilter threshold",
                &mut bloom_settings.prefilter_threshold,
                BloomSettings::DEFAULT_PREFILTER_THRESHOLD,
                0.0..=50.0,
                0.01,
                "CHANGING THIS WILL CREATE A PHYSICALLY INACCURATE IMAGE\nBaseline of the quadratic threshold curve. RGB values under the threshold curve will not contribute to the effect.",
            );
            draw_settings_slider(
                ui,
                "Prefilter threshold softness",
                &mut bloom_settings.prefilter_threshold_softness,
                BloomSettings::DEFAULT_PREFILTER_THRESHOLD_SOFTNESS,
                0.0..=1.0,
                0.01,
                "CHANGING THIS WILL CREATE A PHYSICALLY INACCURATE IMAGE\nControls how much to blend between the thresholded and non-thresholded colors.\n\n• 0.0 = Abrupt threshold, no blending\n• 1.0 = Fully soft threshold",
            );
            ui.vertical_centered(|ui| {
                ui.small("Hover over the slider labels for more info");
            });
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                if ui.button("Reset").clicked() {
                    bloom_settings.reset();
                }
            });
        });
}
