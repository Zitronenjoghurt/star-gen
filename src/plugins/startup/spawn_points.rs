use crate::bundles::point3d::Point3D;
use crate::resources::point_resources::PointResources;
use bevy::prelude::{Assets, Commands, ResMut, StandardMaterial, Vec3};
use rand::Rng;

pub fn spawn_points(
    mut commands: Commands,
    mut point_resources: ResMut<PointResources>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_mesh = point_resources.sphere_mesh();

    let mut rng = rand::rng();
    for x in -5..=5 {
        for y in -5..=5 {
            for z in -5..=5 {
                let red = rng.random::<f32>();
                let green = rng.random::<f32>();
                let blue = rng.random::<f32>();
                let luminance = rng.random_range(10.0..100.0) as f32;

                let material_handle =
                    point_resources.get_material(red, blue, green, luminance, &mut materials);

                let offset_x = rng.random::<f32>() * 4.0 - 1.0;
                let offset_y = rng.random::<f32>() * 4.0 - 1.0;
                let offset_z = rng.random::<f32>() * 4.0 - 1.0;

                let point = Point3D::new(
                    0,
                    Vec3::new(
                        x as f32 + offset_x,
                        y as f32 + offset_y,
                        z as f32 + offset_z,
                    ),
                    0.1,
                    sphere_mesh.clone(),
                    material_handle.clone(),
                );
                commands.spawn(point);
            }
        }
    }
}
