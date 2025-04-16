use crate::resources::point_resources::PointResources;
use crate::resources::selected_point::SelectedPoint;
use bevy::prelude::{
    Assets, Camera3d, ClearColor, Color, Commands, Mesh, ResMut, Sphere, Transform, Vec3,
};
use bevy_panorbit_camera::PanOrbitCamera;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut clear_color: ResMut<ClearColor>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));

    let sphere_mesh = meshes.add(Sphere::new(1.0));
    commands.insert_resource(PointResources::new(sphere_mesh));
    commands.insert_resource(SelectedPoint::default());

    clear_color.0 = Color::BLACK;
}
