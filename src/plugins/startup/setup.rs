use crate::bundles::custom_camera::CustomCamera;
use crate::events::star_generate_cubic::StarGenerateCubicEvent;
use crate::resources::selected_star::SelectedStar;
use crate::resources::star_store::StarStore;
use bevy::prelude::{ClearColor, Color, Commands, EventWriter, ResMut};

pub fn setup(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut star_generate_cubic: EventWriter<StarGenerateCubicEvent>,
) {
    commands.spawn(CustomCamera::default());
    clear_color.0 = Color::BLACK;

    commands.insert_resource(StarStore::default());
    commands.insert_resource(SelectedStar::default());

    star_generate_cubic.send(StarGenerateCubicEvent {
        min_x: -4,
        max_x: 4,
        min_y: -4,
        max_y: 4,
        min_z: -4,
        max_z: 4,
        spread: 100.0,
        offset_factor: 10.0,
        seed: 69,
    });
}
