use crate::events::star_delete::StarDeleteEvent;
use crate::events::star_unselect::StarUnselectEvent;
use crate::resources::selected_star::SelectedStar;
use bevy::prelude::{EventWriter, Res};
use bevy_egui::{egui, EguiContexts};
use egui_extras::{Column, TableBuilder};

pub fn render_selected_star_window(
    mut contexts: EguiContexts,
    selected_star: Res<SelectedStar>,
    mut star_delete_event: EventWriter<StarDeleteEvent>,
    mut star_unselect_event: EventWriter<StarUnselectEvent>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

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
                    body.row(10.0, |mut row| {
                        row.col(|ui| {
                            ui.strong("Position");
                        });
                        row.col(|ui| {
                            let position = data.get_position();
                            ui.label(format!(
                                "({:.2}, {:.2}, {:.2})",
                                position.x, position.y, position.z
                            ));
                        });
                    });
                }
            });

            if let Some(id) = selected_star.get_id() {
                ui.vertical_centered(|ui| {
                    if ui.button("Delete").clicked() {
                        star_unselect_event.send(StarUnselectEvent);
                        star_delete_event.send(StarDeleteEvent::new(id));
                    }
                });
            }
        });
}
