use crate::resources::star_store::StarStore;
use crate::resources::window_manager::WindowManager;
use bevy::prelude::{Res, ResMut};
use bevy_egui::{egui, EguiContexts};

pub fn render_cluster_info_window(
    mut contexts: EguiContexts,
    mut window_manager: ResMut<WindowManager>,
    star_store: Res<StarStore>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    egui::Window::new("Star Cluster")
        .open(&mut window_manager.cluster_info)
        .fade_out(true)
        .fade_in(true)
        .show(ctx, |ui| {
            egui::Grid::new("cluster_properties")
                .striped(true)
                .spacing([40.0, 4.0])
                .show(ui, |ui| {
                    ui.label("Star Count:");
                    ui.label(format!("{}", star_store.get_star_count()));
                    ui.end_row();

                    ui.label("Seed:");
                    ui.label(star_store.get_cluster_method_string());
                    ui.end_row();
                });
        });
}
