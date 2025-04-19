use crate::bundles::point3d::Point3D;
use crate::components::star_data::StarData;
use crate::physics::objects::star::Star;
use bevy::asset::{Assets, Handle};
use bevy::color::{Color, LinearRgba};
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Bundle, Mesh, ResMut, Sphere};

const LUMINANCE_FACTOR: f32 = 10.0;

#[derive(Debug, Bundle)]
pub struct StarBundle {
    data: StarData,
    point3d: Point3D,
}

impl StarBundle {
    pub fn new(
        star: Star,
        id: u64,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let sphere_mesh = meshes.add(Sphere::new(1.0));

        let material_handle = create_new_material(
            star.get_color(),
            star.get_luminosity() as f32 * LUMINANCE_FACTOR,
            materials,
        );

        let point3d = Point3D::new(
            id,
            star.get_position(),
            star.get_radius() as f32,
            sphere_mesh,
            material_handle,
        );

        Self {
            data: StarData::new(star),
            point3d,
        }
    }
}

fn create_new_material(
    color: Color,
    luminance: f32,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    let rgba = color.to_linear();
    materials.add(StandardMaterial {
        base_color: color,
        emissive: LinearRgba::rgb(
            rgba.red * luminance,
            rgba.green * luminance,
            rgba.blue * luminance,
        ),
        metallic: 0.0,
        reflectance: 0.0,
        ..Default::default()
    })
}
