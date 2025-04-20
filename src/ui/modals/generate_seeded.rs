use crate::events::star_cluster_generate::StarClusterGenerateEvent;
use crate::resources::ui::generate_seed_modal_state::GenerateSeedModalState;
use crate::resources::window_manager::WindowManager;
use bevy::prelude::{EventWriter, ResMut};
use bevy_egui::egui::{Id, Modal, TextEdit};
use bevy_egui::{egui, EguiContexts};

pub fn render_generate_seeded_modal(
    mut contexts: EguiContexts,
    mut window_manager: ResMut<WindowManager>,
    mut star_cluster_generate_event: EventWriter<StarClusterGenerateEvent>,
    mut state: ResMut<GenerateSeedModalState>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    Modal::new(Id::new("generate_seeded")).show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Seeded Star Cluster");
        });

        let text_edit = ui.add(
            TextEdit::singleline(&mut state.seed)
                .desired_width(300.0)
                .hint_text("Input seed...")
                .char_limit(64),
        );

        if text_edit.changed() {
            state.validate();
        }

        ui.horizontal(|ui| {
            let generate_button =
                ui.add_enabled(state.method.is_some(), egui::Button::new("Generate"));

            if generate_button.clicked() {
                if let Some(method) = state.method.clone() {
                    star_cluster_generate_event.send(StarClusterGenerateEvent::new(method));
                    window_manager.generate_seeded_modal = false;
                }
            }

            if ui.button("Cancel").clicked() {
                window_manager.generate_seeded_modal = false;
            }

            if state.method.is_none() {
                ui.small("Invalid seed");
            }
        });
    });
}
