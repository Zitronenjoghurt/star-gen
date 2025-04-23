use crate::bundles::point3d::Point3D;
use crate::components::star_data::StarData;
use crate::materials::star::StarMaterial;
use crate::physics::objects::star::Star;
use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{Bundle, Mesh, Mesh3d, ResMut, Sphere};

const LUMINANCE_FACTOR: f32 = 10.0;

#[derive(Bundle)]
pub struct StarBundle {
    data: StarData,
    point3d: Point3D,
    mesh: Mesh3d,
    material: MeshMaterial3d<StarMaterial>,
}

impl StarBundle {
    pub fn new(
        star: Star,
        id: u64,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StarMaterial>>,
        render_distance: f32,
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
            render_distance,
        );

        Self {
            data: StarData::new(star),
            point3d,
            mesh: Mesh3d(sphere_mesh),
            material: MeshMaterial3d(material_handle),
        }
    }
}

fn create_new_material(
    color: Color,
    luminance: f32,
    materials: &mut ResMut<Assets<StarMaterial>>,
) -> Handle<StarMaterial> {
    materials.add(StarMaterial {
        color: color.to_linear(),
    })
}
