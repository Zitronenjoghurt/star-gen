use crate::events::star_delete_all::StarDeleteAllEvent;
use crate::events::star_generate_cubic::StarGenerateCubicEvent;
use crate::resources::ui::generate_cubic_modal_state::GenerateCubicModalState;
use crate::resources::window_manager::WindowManager;
use crate::ui::elements::settings_slider::SettingsSlider;
use bevy::prelude::{EventWriter, ResMut};
use bevy_egui::egui::{Id, Modal, TextEdit};
use bevy_egui::EguiContexts;

pub fn render_generate_cubic_modal(
    mut contexts: EguiContexts,
    mut window_manager: ResMut<WindowManager>,
    mut star_delete_all: EventWriter<StarDeleteAllEvent>,
    mut star_generate_cubic: EventWriter<StarGenerateCubicEvent>,
    mut state: ResMut<GenerateCubicModalState>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    let min_x = state.min_x;
    let max_x = state.max_x;
    let min_y = state.min_y;
    let max_y = state.max_y;
    let min_z = state.min_z;
    let max_z = state.max_z;

    let modal = Modal::new(Id::new("generate_cubic")).show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Cubic Star Cluster");
        });

        SettingsSlider::build()
            .text("min X")
            .tooltip("Defines the starting point of the cubes x-axis edge.")
            .draw(
                ui,
                &mut state.min_x,
                GenerateCubicModalState::DEFAULT_MIN_X,
                -25..=max_x,
                1.0,
            );

        SettingsSlider::build()
            .text("max X")
            .tooltip("Defines the end point of the cubes x-axis edge.")
            .draw(
                ui,
                &mut state.max_x,
                GenerateCubicModalState::DEFAULT_MAX_X,
                min_x..=25,
                1.0,
            );

        SettingsSlider::build()
            .text("min Y")
            .tooltip("Defines the starting point of the cubes y-axis edge.")
            .draw(
                ui,
                &mut state.min_y,
                GenerateCubicModalState::DEFAULT_MIN_Y,
                -25..=max_y,
                1.0,
            );

        SettingsSlider::build()
            .text("max Y")
            .tooltip("Defines the end point of the cubes y-axis edge.")
            .draw(
                ui,
                &mut state.max_y,
                GenerateCubicModalState::DEFAULT_MAX_Y,
                min_y..=25,
                1.0,
            );

        SettingsSlider::build()
            .text("min Z")
            .tooltip("Defines the starting point of the cubes z-axis edge.")
            .draw(
                ui,
                &mut state.min_z,
                GenerateCubicModalState::DEFAULT_MIN_Z,
                -25..=max_z,
                1.0,
            );

        SettingsSlider::build()
            .text("max Z")
            .tooltip("Defines the end point of the cubes z-axis edge.")
            .draw(
                ui,
                &mut state.max_z,
                GenerateCubicModalState::DEFAULT_MAX_Z,
                min_z..=25,
                1.0,
            );

        SettingsSlider::build()
            .text("Spread")
            .tooltip("How spread apart the stars will spawn.")
            .logarithmic(true)
            .draw(
                ui,
                &mut state.spread,
                GenerateCubicModalState::DEFAULT_SPREAD,
                1.0..=1_000_000.0,
                0.1,
            );

        SettingsSlider::build()
            .text("Offset factor")
            .tooltip("How far stars can randomly deviate from their strict spawn position.")
            .draw(
                ui,
                &mut state.offset_factor,
                GenerateCubicModalState::DEFAULT_OFFSET_FACTOR,
                0.0..=10.0,
                0.1,
            );

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Seed:");

            let mut seed_text = state.edit_seed.clone();
            let text_edit = ui.add(
                TextEdit::singleline(&mut seed_text)
                    .desired_width(100.0)
                    .hint_text("seed")
                    .char_limit(16),
            );

            if text_edit.changed() {
                state.edit_seed = seed_text.clone();
            }

            if text_edit.lost_focus() {
                state.set_seed(seed_text);
            }

            if ui.button("Regenerate").clicked() {
                state.regenerate_seed();
            }
        });

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            if ui.button("Generate").clicked() {
                if let Some(generate_event) = state.get_event() {
                    star_delete_all.send(StarDeleteAllEvent);
                    star_generate_cubic.send(generate_event);
                    window_manager.generate_cubic_modal = false;
                }
            }
            if ui.button("Cancel").clicked() {
                window_manager.generate_cubic_modal = false;
            }
            if ui.button("Reset").clicked() {
                state.reset();
            }
        });
    });

    if modal.should_close() {
        window_manager.generate_cubic_modal = false;
    }
}
