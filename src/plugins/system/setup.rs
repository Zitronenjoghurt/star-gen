use crate::bundles::custom_camera::CustomCamera;
use crate::resources::selected_star::SelectedStar;
use crate::resources::star_store::StarStore;
use bevy::prelude::{ClearColor, Color, Commands, ResMut};

pub fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    commands.spawn(CustomCamera::default());
    clear_color.0 = Color::BLACK;

    commands.insert_resource(StarStore::default());
    commands.insert_resource(SelectedStar::default());
}
