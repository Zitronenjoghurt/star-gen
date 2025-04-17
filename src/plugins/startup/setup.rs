use crate::bundles::custom_camera::CustomCamera;
use crate::resources::point_resources::PointResources;
use crate::resources::selected_point::SelectedPoint;
use crate::resources::simulation_settings::SimulationSettings;
use bevy::prelude::{Assets, ClearColor, Color, Commands, Mesh, ResMut, Sphere};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut clear_color: ResMut<ClearColor>,
) {
    commands.spawn(CustomCamera::default());
    clear_color.0 = Color::BLACK;

    let sphere_mesh = meshes.add(Sphere::new(1.0));
    commands.insert_resource(PointResources::new(sphere_mesh));
    commands.insert_resource(SelectedPoint::default());
    commands.insert_resource(SimulationSettings::default());
}
