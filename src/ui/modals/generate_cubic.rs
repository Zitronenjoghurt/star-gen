use crate::resources::window_manager::WindowManager;
use bevy::prelude::ResMut;
use bevy_egui::egui::{Context, Id, Modal, Sides};

pub fn draw_generate_cubic_modal(ctx: &mut Context, window_manager: &mut ResMut<WindowManager>) {
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
