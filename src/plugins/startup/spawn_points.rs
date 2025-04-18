use crate::bundles::point3d::Point3D;
use crate::physics::objects::star::Star;
use crate::resources::point_resources::PointResources;
use bevy::prelude::{Assets, Commands, ResMut, StandardMaterial, Vec3};
use rand::Rng;

const TOTAL_SPREAD: f32 = 50.0;
const LUMINANCE_FACTOR: f32 = 1.0;
const CUBIC_COUNT: i32 = 4;
const OFFSET_FACTOR: f32 = 4.0;

pub fn spawn_points(
    mut commands: Commands,
    mut point_resources: ResMut<PointResources>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_mesh = point_resources.sphere_mesh();

    let mut rng = rand::rng();
    let mut new_id: u64 = 0;
    for x in -CUBIC_COUNT..=CUBIC_COUNT {
        for y in -CUBIC_COUNT..=CUBIC_COUNT {
            for z in -CUBIC_COUNT..=CUBIC_COUNT {
                let star = Star::new_random(&mut rng);
                let color = star.get_color().to_linear();

                let material_handle = point_resources.get_material(
                    color.red,
                    color.green,
                    color.blue,
                    star.get_luminosity() as f32 * LUMINANCE_FACTOR,
                    &mut materials,
                );

                let offset_x = rng.random::<f32>() * OFFSET_FACTOR - 1.0;
                let offset_y = rng.random::<f32>() * OFFSET_FACTOR - 1.0;
                let offset_z = rng.random::<f32>() * OFFSET_FACTOR - 1.0;

                let point = Point3D::new(
                    new_id,
                    Vec3::new(
                        (x as f32 + offset_x) * TOTAL_SPREAD,
                        (y as f32 + offset_y) * TOTAL_SPREAD,
                        (z as f32 + offset_z) * TOTAL_SPREAD,
                    ),
                    star.get_radius() as f32,
                    sphere_mesh.clone(),
                    material_handle.clone(),
                );
                commands.spawn(point);

                new_id += 1;
            }
        }
    }
}
