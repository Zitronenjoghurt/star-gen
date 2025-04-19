use crate::resources::window_manager::WindowManager;
use bevy::diagnostic::{
    DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::{Res, ResMut};
use bevy_egui::{egui, EguiContexts};

pub fn render_diagnostics_window(
    mut contexts: EguiContexts,
    mut window_manager: ResMut<WindowManager>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    let fps = diagnostics.get_measurement(&FrameTimeDiagnosticsPlugin::FPS);
    let frame_time = diagnostics.get_measurement(&FrameTimeDiagnosticsPlugin::FRAME_TIME);
    let entity_count = diagnostics.get_measurement(&EntityCountDiagnosticsPlugin::ENTITY_COUNT);

    egui::Window::new("Diagnostics")
        .open(&mut window_manager.diagnostics)
        .fade_out(true)
        .fade_in(true)
        .show(ctx, |ui| {
            if let Some(fps) = fps {
                ui.label(format!("FPS: {:.00}", fps.value));
            }
            if let Some(frame_time) = frame_time {
                ui.label(format!("FT: {:.00}ms", frame_time.value));
            }
            if let Some(entity_count) = entity_count {
                ui.label(format!("Entity Count: {}", entity_count.value));
            }
        });
}
