use crate::bundles::point3d::Point3D;
use crate::resources::point_resources::PointResources;
use bevy::prelude::{Assets, Color, Commands, LinearRgba, Res, ResMut, StandardMaterial, Vec3};

pub fn spawn_points(
    mut commands: Commands,
    point_resources: Res<PointResources>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_mesh = point_resources.sphere_mesh();

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(1.0, 0.0, 0.0),
        emissive: LinearRgba::new(0.5, 0.0, 0.0, 1.0),
        ..Default::default()
    });

    let point = Point3D::new(
        0,
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        sphere_mesh.clone(),
        material_handle,
    );

    commands.spawn(point);
}
