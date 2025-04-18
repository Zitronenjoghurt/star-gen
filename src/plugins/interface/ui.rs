use crate::resources::selected_star::SelectedStar;
use crate::resources::simulation_settings::{BloomSettings, WireframeSettings};
use crate::resources::window_manager::WindowManager;
use bevy::diagnostic::{
    DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::{Res, ResMut};
use bevy_egui::{egui, EguiContexts};
use egui_extras::{Column, TableBuilder};

pub fn user_interface(
    mut contexts: EguiContexts,
    mut window_manager: ResMut<WindowManager>,
    diagnostics: Res<DiagnosticsStore>,
    selected_star: Res<SelectedStar>,
    mut bloom_settings: ResMut<BloomSettings>,
    mut wireframe_settings: ResMut<WireframeSettings>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Windows", |ui| {
                ui.checkbox(&mut window_manager.settings, "Settings");
                ui.checkbox(&mut window_manager.diagnostics, "Diagnostics");
            });
            ui.menu_button("Settings", |ui| {
                ui.checkbox(&mut bloom_settings.active, "Bloom Effect");
                ui.checkbox(&mut wireframe_settings.active, "Wireframe Mode");
            });
        });
    });

    if window_manager.settings {
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

    if window_manager.diagnostics {
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

    if selected_star.get_id().is_some() {
        egui::Window::new("Selected Star")
            .title_bar(true)
            .resizable(false)
            .show(ctx, |ui| {
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::auto());

                table.body(|mut body| {
                    if let Some(id) = selected_star.get_id() {
                        body.row(10.0, |mut row| {
                            row.col(|ui| {
                                ui.strong("ID");
                            });
                            row.col(|ui| {
                                ui.label(format!("{id}"));
                            });
                        });
                    }
                    if let Some(data) = selected_star.get_data() {
                        body.row(10.0, |mut row| {
                            row.col(|ui| {
                                ui.strong("Class");
                            });
                            row.col(|ui| {
                                ui.label(format!("{:?}", data.get_stellar_class()));
                            });
                        });
                        body.row(10.0, |mut row| {
                            row.col(|ui| {
                                ui.strong("Mass");
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}M☉", data.get_mass()));
                            });
                        });
                        body.row(10.0, |mut row| {
                            row.col(|ui| {
                                ui.strong("Surface Temp.");
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}K", data.get_surface_temperature()));
                            });
                        });
                        body.row(10.0, |mut row| {
                            row.col(|ui| {
                                ui.strong("Radius");
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}R☉", data.get_radius()));
                            });
                        });
                        body.row(10.0, |mut row| {
                            row.col(|ui| {
                                ui.strong("Luminosity");
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}L☉", data.get_luminosity()));
                            });
                        });
                    }
                });
            });
    }
}
