use crate::resources::simulation_settings::{BloomSettings, WireframeSettings};
use crate::resources::window_manager::WindowManager;
use bevy::diagnostic::{
    DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::{Res, ResMut};
use bevy_egui::{egui, EguiContexts};

pub fn user_interface(
    mut contexts: EguiContexts,
    mut window_manager: ResMut<WindowManager>,
    diagnostics: Res<DiagnosticsStore>,
    mut bloom_settings: ResMut<BloomSettings>,
    mut wireframe_settings: ResMut<WireframeSettings>,
) {
    egui::TopBottomPanel::top("top_panel").show(contexts.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Windows", |ui| {
                ui.checkbox(&mut window_manager.settings, "Settings");
                ui.checkbox(&mut window_manager.diagnostics, "Diagnostics");
            });
            ui.menu_button("Tools", |ui| {
                ui.checkbox(&mut bloom_settings.active, "Bloom Effect");
                ui.checkbox(&mut wireframe_settings.active, "Wireframe Mode");
            });
        });
    });

    if window_manager.settings {
        egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
            ui.add(egui::Checkbox::new(&mut bloom_settings.active, "Bloom"));
            ui.add(egui::Checkbox::new(
                &mut wireframe_settings.active,
                "Wireframe",
            ));
        });
    }

    if window_manager.diagnostics {
        let fps = diagnostics.get_measurement(&FrameTimeDiagnosticsPlugin::FPS);
        let frame_time = diagnostics.get_measurement(&FrameTimeDiagnosticsPlugin::FRAME_TIME);
        let entity_count = diagnostics.get_measurement(&EntityCountDiagnosticsPlugin::ENTITY_COUNT);

        egui::Window::new("Diagnostics").show(contexts.ctx_mut(), |ui| {
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
}
