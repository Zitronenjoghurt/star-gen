use crate::components::selectable_point::SelectablePoint;
use bevy::prelude::{
    Bundle, GlobalTransform, Handle, InheritedVisibility, Mesh, Mesh3d, MeshMaterial3d,
    StandardMaterial, Transform, Vec3, ViewVisibility, Visibility,
};

#[derive(Debug, Default, Bundle)]
pub struct Point3D {
    selectable_point: SelectablePoint,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl Point3D {
    pub fn new(
        id: u64,
        position: Vec3,
        scale: f32,
        mesh_handle: Handle<Mesh>,
        material_handle: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            selectable_point: SelectablePoint::new(id, false),
            mesh: Mesh3d(mesh_handle),
            material: MeshMaterial3d(material_handle),
            transform: Transform::from_translation(position).with_scale(Vec3::splat(scale)),
            ..Default::default()
        }
    }
}
