use crate::resources::window_manager::WindowManager;
use bevy::prelude::ResMut;
use bevy_egui::egui::{Id, Modal, Sides};
use bevy_egui::EguiContexts;

pub fn render_generate_cubic_modal(
    mut contexts: EguiContexts,
    mut window_manager: ResMut<WindowManager>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    let modal = Modal::new(Id::new("generate_cubic")).show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Cubic Star Cluster");
        });
        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui.button("Cancel").clicked() {
                    ui.close_menu();
                }
            },
        );
    });

    if modal.should_close() {
        window_manager.generate_cubic_modal = false;
    }
}
