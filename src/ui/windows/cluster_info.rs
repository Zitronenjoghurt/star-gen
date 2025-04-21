use crate::events::star_clicked::StarClickedEvent;
use crate::resources::star_store::StarStore;
use crate::resources::ui::cluster_info_window_state::{
    ClusterInfoSortType, ClusterInfoWindowState,
};
use crate::resources::window_manager::WindowManager;
use crate::types::sort_direction::SortDirection;
use bevy::prelude::{EventWriter, Res, ResMut};
use bevy_egui::{egui, EguiContexts};
use egui_extras::{Column, TableBuilder};

pub fn render_cluster_info_window(
    mut contexts: EguiContexts,
    mut window_manager: ResMut<WindowManager>,
    mut star_clicked_event: EventWriter<StarClickedEvent>,
    mut state: ResMut<ClusterInfoWindowState>,
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

            ui.add_space(10.0);

            let table = TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto());

            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("ID");
                    });
                    header.col(|ui| {
                        ui.strong("Class");
                    });
                    header.col(|ui| {
                        let text = if state.sort_type == ClusterInfoSortType::Mass {
                            match state.sort_direction {
                                SortDirection::Ascending => "Mass 🔽",
                                SortDirection::Descending => "Mass 🔼",
                            }
                        } else {
                            "Mass"
                        };

                        if ui.strong(text).clicked() {
                            state.on_type_clicked(ClusterInfoSortType::Mass);
                        };
                    });
                    header.col(|ui| {
                        let text = if state.sort_type == ClusterInfoSortType::SurfaceTemperature {
                            match state.sort_direction {
                                SortDirection::Ascending => "Surf. Tmp. 🔽",
                                SortDirection::Descending => "Surf. Tmp. 🔼",
                            }
                        } else {
                            "Surf. Tmp."
                        };

                        if ui.strong(text).clicked() {
                            state.on_type_clicked(ClusterInfoSortType::SurfaceTemperature);
                        };
                    });
                    header.col(|ui| {
                        let text = if state.sort_type == ClusterInfoSortType::Radius {
                            match state.sort_direction {
                                SortDirection::Ascending => "Radius 🔽",
                                SortDirection::Descending => "Radius 🔼",
                            }
                        } else {
                            "Radius"
                        };

                        if ui.strong(text).clicked() {
                            state.on_type_clicked(ClusterInfoSortType::Radius);
                        };
                    });
                    header.col(|ui| {
                        let text = if state.sort_type == ClusterInfoSortType::Luminosity {
                            match state.sort_direction {
                                SortDirection::Ascending => "Luminosity 🔽",
                                SortDirection::Descending => "Luminosity 🔼",
                            }
                        } else {
                            "Luminosity"
                        };

                        if ui.strong(text).clicked() {
                            state.on_type_clicked(ClusterInfoSortType::Luminosity);
                        };
                    });
                    header.col(|ui| {
                        ui.strong("Actions");
                    });
                })
                .body(|body| {
                    body.rows(10.0, star_store.get_star_count(), |mut row| {
                        let ascending = state.sort_direction == SortDirection::Ascending;
                        if let Some(id) = match state.sort_type {
                            ClusterInfoSortType::None => Some(row.index() as u64 + 1),
                            ClusterInfoSortType::Mass => {
                                star_store.get_id_sorted_by_mass(row.index(), ascending)
                            }
                            ClusterInfoSortType::Radius => {
                                star_store.get_id_sorted_radius(row.index(), ascending)
                            }
                            ClusterInfoSortType::Luminosity => {
                                star_store.get_id_sorted_luminosity(row.index(), ascending)
                            }
                            ClusterInfoSortType::SurfaceTemperature => {
                                star_store.get_id_sorted_surface_temperature(row.index(), ascending)
                            }
                        } {
                            if let Some(star) = star_store.get_star(id) {
                                row.col(|ui| {
                                    ui.label(format!("{id}"));
                                });
                                row.col(|ui| {
                                    ui.label(format!("{:?}", star.get_stellar_class()));
                                });
                                row.col(|ui| {
                                    ui.label(format!("{:.2} M☉", star.get_mass()));
                                });
                                row.col(|ui| {
                                    ui.label(format!("{:.2} K", star.get_surface_temperature()));
                                });
                                row.col(|ui| {
                                    ui.label(format!("{:.2} R☉", star.get_radius()));
                                });
                                row.col(|ui| {
                                    ui.label(format!("{:.2} L☉", star.get_luminosity()));
                                });
                                row.col(|ui| {
                                    if ui.button("🔍").clicked() {
                                        star_clicked_event.send(StarClickedEvent::new(id));
                                    }
                                });
                            }
                        }
                    })
                });
        });
}
