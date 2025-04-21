use crate::events::star_delete::StarDeleteEvent;
use crate::resources::selected_star::SelectedStar;
use bevy::prelude::{EventWriter, Res};
use bevy_egui::{egui, EguiContexts};

pub fn render_selected_star_window(
    mut contexts: EguiContexts,
    selected_star: Res<SelectedStar>,
    mut star_delete_event: EventWriter<StarDeleteEvent>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    egui::Window::new("Selected Star")
        .title_bar(true)
        .resizable(false)
        .show(ctx, |ui| {
            egui::Grid::new("star_properties")
                .striped(true)
                .spacing([40.0, 4.0])
                .show(ui, |ui| {
                    if let Some(id) = selected_star.get_id() {
                        ui.strong("ID");
                        ui.label(format!("{id}"));
                        ui.end_row();
                    }

                    if let Some(data) = selected_star.get_data() {
                        ui.strong("Class");
                        ui.label(format!("{:?}", data.get_stellar_class()));
                        ui.end_row();

                        ui.strong("Mass");
                        ui.label(format!("{:.2} M☉", data.get_mass()));
                        ui.end_row();

                        ui.strong("Surface Temp.");
                        ui.label(format!("{:.2}K", data.get_surface_temperature()));
                        ui.end_row();

                        ui.strong("Radius");
                        ui.label(format!("{:.2} R☉", data.get_radius()));
                        ui.end_row();

                        ui.strong("Luminosity");
                        ui.label(format!("{:.2} L☉", data.get_luminosity()));
                        ui.end_row();

                        let position = data.get_position();
                        ui.strong("Position");
                        ui.label(format!(
                            "{:.2} | {:.2} | {:.2}",
                            position.x, position.y, position.z
                        ));
                        ui.end_row();
                    }
                });

            if let Some(id) = selected_star.get_id() {
                ui.add_space(15.0);
                if ui.button("Delete").clicked() {
                    star_delete_event.send(StarDeleteEvent::new(id));
                }
            }
        });
}
